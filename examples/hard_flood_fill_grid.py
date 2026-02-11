"""Flood fill on a 1D-encoded grid using iterative stack."""


def make_grid(rows: int, cols: int, val: int) -> list[int]:
    """Create a flat grid initialized to val."""
    grid: list[int] = []
    i: int = 0
    total: int = rows * cols
    while i < total:
        grid.append(val)
        i = i + 1
    return grid


def get_cell(grid: list[int], cols: int, r: int, c: int) -> int:
    """Get cell value from flat grid."""
    return grid[r * cols + c]


def set_cell(grid: list[int], cols: int, r: int, c: int, val: int) -> int:
    """Set cell value in flat grid. Returns val."""
    grid[r * cols + c] = val
    return val


def flood_fill(grid: list[int], rows: int, cols: int, sr: int, sc: int, new_color: int) -> int:
    """Flood fill starting at (sr, sc). Returns number of cells filled."""
    old_color: int = get_cell(grid, cols, sr, sc)
    if old_color == new_color:
        return 0
    stack_r: list[int] = [sr]
    stack_c: list[int] = [sc]
    filled: int = 0
    while len(stack_r) > 0:
        top: int = len(stack_r) - 1
        cr: int = stack_r[top]
        cc: int = stack_c[top]
        stack_r.pop()
        stack_c.pop()
        if cr < 0 or cr >= rows or cc < 0 or cc >= cols:
            filled = filled + 0
        elif get_cell(grid, cols, cr, cc) != old_color:
            filled = filled + 0
        else:
            set_cell(grid, cols, cr, cc, new_color)
            filled = filled + 1
            stack_r.append(cr - 1)
            stack_c.append(cc)
            stack_r.append(cr + 1)
            stack_c.append(cc)
            stack_r.append(cr)
            stack_c.append(cc - 1)
            stack_r.append(cr)
            stack_c.append(cc + 1)
    return filled


def count_value(grid: list[int], val: int) -> int:
    """Count cells with given value."""
    count: int = 0
    i: int = 0
    n: int = len(grid)
    while i < n:
        if grid[i] == val:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test flood fill."""
    passed: int = 0

    g1: list[int] = make_grid(3, 3, 1)
    set_cell(g1, 3, 1, 1, 2)
    filled: int = flood_fill(g1, 3, 3, 0, 0, 3)
    if filled == 8:
        passed = passed + 1

    if get_cell(g1, 3, 1, 1) == 2:
        passed = passed + 1

    g2: list[int] = make_grid(2, 2, 0)
    filled2: int = flood_fill(g2, 2, 2, 0, 0, 5)
    if filled2 == 4:
        passed = passed + 1

    if count_value(g2, 5) == 4:
        passed = passed + 1

    g3: list[int] = make_grid(1, 1, 7)
    filled3: int = flood_fill(g3, 1, 1, 0, 0, 7)
    if filled3 == 0:
        passed = passed + 1

    g4: list[int] = make_grid(3, 3, 0)
    set_cell(g4, 3, 0, 1, 1)
    set_cell(g4, 3, 1, 0, 1)
    filled4: int = flood_fill(g4, 3, 3, 0, 0, 9)
    if filled4 == 1:
        passed = passed + 1

    return passed
