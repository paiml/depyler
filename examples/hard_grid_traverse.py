"""Grid traversal operations.

Implements various grid traversal patterns on flat arrays
representing 2D grids with computed row/column offsets.
"""


def grid_get(grid: list[int], cols: int, row: int, col: int) -> int:
    """Get element at (row, col) from flat grid with given column count."""
    idx: int = row * cols + col
    return grid[idx]


def grid_set(grid: list[int], cols: int, row: int, col: int, value: int) -> int:
    """Set element at (row, col) in flat grid. Returns the value set."""
    idx: int = row * cols + col
    grid[idx] = value
    return value


def traverse_row_major(grid: list[int], rows: int, cols: int) -> int:
    """Sum all elements in row-major order."""
    total: int = 0
    r: int = 0
    while r < rows:
        c: int = 0
        while c < cols:
            idx: int = r * cols + c
            total = total + grid[idx]
            c = c + 1
        r = r + 1
    return total


def traverse_column_major(grid: list[int], rows: int, cols: int) -> int:
    """Sum all elements in column-major order."""
    total: int = 0
    c: int = 0
    while c < cols:
        r: int = 0
        while r < rows:
            idx: int = r * cols + c
            total = total + grid[idx]
            r = r + 1
        c = c + 1
    return total


def traverse_diagonal(grid: list[int], size: int) -> int:
    """Sum main diagonal elements of a square grid."""
    total: int = 0
    i: int = 0
    while i < size:
        idx: int = i * size + i
        total = total + grid[idx]
        i = i + 1
    return total


def count_neighbors(grid: list[int], rows: int, cols: int, row: int, col: int) -> int:
    """Count non-zero 4-neighbors of a cell."""
    count: int = 0
    if row > 0:
        up_idx: int = (row - 1) * cols + col
        if grid[up_idx] != 0:
            count = count + 1
    if row < rows - 1:
        down_idx: int = (row + 1) * cols + col
        if grid[down_idx] != 0:
            count = count + 1
    if col > 0:
        left_idx: int = row * cols + (col - 1)
        if grid[left_idx] != 0:
            count = count + 1
    if col < cols - 1:
        right_idx: int = row * cols + (col + 1)
        if grid[right_idx] != 0:
            count = count + 1
    return count


def test_module() -> int:
    """Test grid traversal operations."""
    ok: int = 0

    grid: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9]

    val: int = grid_get(grid, 3, 1, 1)
    if val == 5:
        ok = ok + 1

    row_sum: int = traverse_row_major(grid, 3, 3)
    col_sum: int = traverse_column_major(grid, 3, 3)
    if row_sum == 45 and col_sum == 45:
        ok = ok + 1

    diag: int = traverse_diagonal(grid, 3)
    if diag == 15:
        ok = ok + 1

    nbrs: int = count_neighbors(grid, 3, 3, 1, 1)
    if nbrs == 4:
        ok = ok + 1

    return ok
