def make_maze(rows: int, cols: int) -> list[list[int]]:
    maze: list[list[int]] = []
    r: int = 0
    while r < rows:
        row: list[int] = []
        c: int = 0
        while c < cols:
            row.append(1)
            c = c + 1
        maze.append(row)
        r = r + 1
    return maze

def set_path(maze: list[list[int]], r: int, c: int) -> int:
    row: list[int] = maze[r]
    row[c] = 0
    return 1

def is_wall(maze: list[list[int]], r: int, c: int, rows: int, cols: int) -> int:
    if r < 0 or r >= rows or c < 0 or c >= cols:
        return 1
    row: list[int] = maze[r]
    v: int = row[c]
    return v

def count_paths(maze: list[list[int]], rows: int, cols: int) -> int:
    count: int = 0
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            row: list[int] = maze[r]
            v: int = row[c]
            if v == 0:
                count = count + 1
            c = c + 1
        r = r + 1
    return count

def neighbors(r: int, c: int, rows: int, cols: int) -> list[list[int]]:
    result: list[list[int]] = []
    dirs: list[list[int]] = [[0-1, 0], [1, 0], [0, 0-1], [0, 1]]
    i: int = 0
    while i < 4:
        d: list[int] = dirs[i]
        nr: int = r + d[0]
        nc: int = c + d[1]
        if nr >= 0 and nr < rows and nc >= 0 and nc < cols:
            result.append([nr, nc])
        i = i + 1
    return result

def manhattan_dist(r1: int, c1: int, r2: int, c2: int) -> int:
    dr: int = r1 - r2
    if dr < 0:
        dr = 0 - dr
    dc: int = c1 - c2
    if dc < 0:
        dc = 0 - dc
    return dr + dc

def solve_path_exists(maze: list[list[int]], sr: int, sc: int, er: int, ec: int, rows: int, cols: int) -> int:
    visited: list[list[int]] = make_maze(rows, cols)
    stack_r: list[int] = [sr]
    stack_c: list[int] = [sc]
    vr: list[int] = visited[sr]
    vr[sc] = 2
    while len(stack_r) > 0:
        ns: int = len(stack_r)
        cr: int = stack_r[ns - 1]
        cc: int = stack_c[ns - 1]
        stack_r.pop()
        stack_c.pop()
        if cr == er and cc == ec:
            return 1
        nbrs: list[list[int]] = neighbors(cr, cc, rows, cols)
        ni: int = 0
        nn: int = len(nbrs)
        while ni < nn:
            nb: list[int] = nbrs[ni]
            nr: int = nb[0]
            nc: int = nb[1]
            mrow: list[int] = maze[nr]
            mv: int = mrow[nc]
            vrow: list[int] = visited[nr]
            vv: int = vrow[nc]
            if mv == 0 and vv != 2:
                vrow[nc] = 2
                stack_r.append(nr)
                stack_c.append(nc)
            ni = ni + 1
    return 0

def test_module() -> int:
    passed: int = 0
    m: list[list[int]] = make_maze(5, 5)
    w: int = is_wall(m, 0, 0, 5, 5)
    if w == 1:
        passed = passed + 1
    set_path(m, 0, 0)
    w2: int = is_wall(m, 0, 0, 5, 5)
    if w2 == 0:
        passed = passed + 1
    cp: int = count_paths(m, 5, 5)
    if cp == 1:
        passed = passed + 1
    md: int = manhattan_dist(0, 0, 4, 4)
    if md == 8:
        passed = passed + 1
    nb: list[list[int]] = neighbors(2, 2, 5, 5)
    nnb: int = len(nb)
    if nnb == 4:
        passed = passed + 1
    return passed
