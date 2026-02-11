def make_grid(rows: int, cols: int) -> list[list[int]]:
    grid: list[list[int]] = []
    r: int = 0
    while r < rows:
        row: list[int] = []
        c: int = 0
        while c < cols:
            row.append(0)
            c = c + 1
        grid.append(row)
        r = r + 1
    return grid

def set_cell(grid: list[list[int]], r: int, c: int, val: int) -> int:
    row: list[int] = grid[r]
    row[c] = val
    return 1

def count_neighbors(grid: list[list[int]], r: int, c: int, rows: int, cols: int) -> int:
    count: int = 0
    dr: int = 0 - 1
    while dr <= 1:
        dc: int = 0 - 1
        while dc <= 1:
            if dr != 0 or dc != 0:
                nr: int = r + dr
                nc: int = c + dc
                if nr >= 0 and nr < rows and nc >= 0 and nc < cols:
                    row: list[int] = grid[nr]
                    v: int = row[nc]
                    count = count + v
            dc = dc + 1
        dr = dr + 1
    return count

def step_life(grid: list[list[int]], rows: int, cols: int) -> list[list[int]]:
    new_grid: list[list[int]] = make_grid(rows, cols)
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            neighbors: int = count_neighbors(grid, r, c, rows, cols)
            curr_row: list[int] = grid[r]
            alive: int = curr_row[c]
            new_row: list[int] = new_grid[r]
            if alive == 1:
                if neighbors == 2 or neighbors == 3:
                    new_row[c] = 1
                else:
                    new_row[c] = 0
            else:
                if neighbors == 3:
                    new_row[c] = 1
            c = c + 1
        r = r + 1
    return new_grid

def count_alive(grid: list[list[int]], rows: int, cols: int) -> int:
    count: int = 0
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            row: list[int] = grid[r]
            count = count + row[c]
            c = c + 1
        r = r + 1
    return count

def test_module() -> int:
    passed: int = 0
    g: list[list[int]] = make_grid(5, 5)
    ca: int = count_alive(g, 5, 5)
    if ca == 0:
        passed = passed + 1
    set_cell(g, 1, 2, 1)
    set_cell(g, 2, 2, 1)
    set_cell(g, 3, 2, 1)
    ca2: int = count_alive(g, 5, 5)
    if ca2 == 3:
        passed = passed + 1
    n: int = count_neighbors(g, 2, 2, 5, 5)
    if n == 2:
        passed = passed + 1
    g2: list[list[int]] = step_life(g, 5, 5)
    ca3: int = count_alive(g2, 5, 5)
    if ca3 == 3:
        passed = passed + 1
    r2: list[int] = g2[2]
    v21: int = r2[1]
    if v21 == 1:
        passed = passed + 1
    return passed
