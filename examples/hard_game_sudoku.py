def make_sudoku() -> list[list[int]]:
    grid: list[list[int]] = []
    r: int = 0
    while r < 9:
        row: list[int] = []
        c: int = 0
        while c < 9:
            row.append(0)
            c = c + 1
        grid.append(row)
        r = r + 1
    return grid

def valid_in_row(grid: list[list[int]], r: int, num: int) -> int:
    row: list[int] = grid[r]
    c: int = 0
    while c < 9:
        v: int = row[c]
        if v == num:
            return 0
        c = c + 1
    return 1

def valid_in_col(grid: list[list[int]], c: int, num: int) -> int:
    r: int = 0
    while r < 9:
        row: list[int] = grid[r]
        v: int = row[c]
        if v == num:
            return 0
        r = r + 1
    return 1

def valid_in_box(grid: list[list[int]], r: int, c: int, num: int) -> int:
    br: int = (r // 3) * 3
    bc: int = (c // 3) * 3
    dr: int = 0
    while dr < 3:
        dc: int = 0
        while dc < 3:
            row: list[int] = grid[br + dr]
            v: int = row[bc + dc]
            if v == num:
                return 0
            dc = dc + 1
        dr = dr + 1
    return 1

def is_valid_placement(grid: list[list[int]], r: int, c: int, num: int) -> int:
    vr: int = valid_in_row(grid, r, num)
    if vr == 0:
        return 0
    vc: int = valid_in_col(grid, c, num)
    if vc == 0:
        return 0
    vb: int = valid_in_box(grid, r, c, num)
    return vb

def count_filled(grid: list[list[int]]) -> int:
    count: int = 0
    r: int = 0
    while r < 9:
        c: int = 0
        while c < 9:
            row: list[int] = grid[r]
            v: int = row[c]
            if v != 0:
                count = count + 1
            c = c + 1
        r = r + 1
    return count

def test_module() -> int:
    passed: int = 0
    g: list[list[int]] = make_sudoku()
    cf: int = count_filled(g)
    if cf == 0:
        passed = passed + 1
    r0: list[int] = g[0]
    r0[0] = 5
    vr: int = valid_in_row(g, 0, 5)
    if vr == 0:
        passed = passed + 1
    vr2: int = valid_in_row(g, 0, 3)
    if vr2 == 1:
        passed = passed + 1
    vc: int = valid_in_col(g, 0, 5)
    if vc == 0:
        passed = passed + 1
    vp: int = is_valid_placement(g, 1, 1, 5)
    if vp == 0:
        passed = passed + 1
    return passed
