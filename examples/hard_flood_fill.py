"""Flood fill on a 2D grid (flattened to 1D).

Tests: fill region, count cells, grid copy.
"""


def grid_get(grid: list[int], cols: int, r: int, c: int) -> int:
    """Get value at row r, col c in flattened grid."""
    return grid[r * cols + c]


def copy_grid(grid: list[int]) -> list[int]:
    """Copy a grid."""
    result: list[int] = []
    i: int = 0
    while i < len(grid):
        result.append(grid[i])
        i = i + 1
    return result


def count_value(grid: list[int], val: int) -> int:
    """Count cells with a specific value."""
    count: int = 0
    i: int = 0
    while i < len(grid):
        if grid[i] == val:
            count = count + 1
        i = i + 1
    return count


def flood_fill_row(grid: list[int], rows: int, cols: int, old_color: int, new_color: int) -> list[int]:
    """Fill connected region row by row using scanning approach."""
    result: list[int] = copy_grid(grid)
    changed: int = 1
    while changed == 1:
        changed = 0
        r: int = 0
        while r < rows:
            c: int = 0
            while c < cols:
                if result[r * cols + c] == old_color:
                    has_new_neighbor: int = 0
                    if r > 0 and result[(r - 1) * cols + c] == new_color:
                        has_new_neighbor = 1
                    if r < rows - 1 and result[(r + 1) * cols + c] == new_color:
                        has_new_neighbor = 1
                    if c > 0 and result[r * cols + c - 1] == new_color:
                        has_new_neighbor = 1
                    if c < cols - 1 and result[r * cols + c + 1] == new_color:
                        has_new_neighbor = 1
                    if has_new_neighbor == 1:
                        result[r * cols + c] = new_color
                        changed = 1
                c = c + 1
            r = r + 1
    return result


def flood_fill_start(grid: list[int], rows: int, cols: int, sr: int, sc: int, new_color: int) -> list[int]:
    """Flood fill from starting point."""
    old_color: int = grid[sr * cols + sc]
    if old_color == new_color:
        return copy_grid(grid)
    result: list[int] = copy_grid(grid)
    result[sr * cols + sc] = new_color
    return flood_fill_row(result, rows, cols, old_color, new_color)


def test_module() -> None:
    g: list[int] = [1, 1, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 1]
    assert grid_get(g, 4, 0, 0) == 1
    assert grid_get(g, 4, 1, 2) == 0
    filled: list[int] = flood_fill_start(g, 4, 4, 0, 0, 2)
    assert filled[0] == 2
    assert filled[1] == 2
    assert filled[4] == 2
    assert filled[5] == 2
    assert count_value(filled, 2) == 4
    assert count_value(filled, 1) == 4
