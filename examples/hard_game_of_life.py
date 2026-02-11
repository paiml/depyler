"""Conway's Game of Life step using flat grid representation."""


def get_cell(grid: list[int], width: int, row: int, col: int) -> int:
    """Get cell value at (row, col) from flat grid."""
    idx: int = row * width + col
    return grid[idx]


def count_neighbors(grid: list[int], width: int, height: int, row: int, col: int) -> int:
    """Count live neighbors of cell at (row, col)."""
    count: int = 0
    dr: int = -1
    while dr <= 1:
        dc: int = -1
        while dc <= 1:
            if dr == 0 and dc == 0:
                dc = dc + 1
                continue
            nr: int = row + dr
            nc: int = col + dc
            if nr >= 0 and nr < height and nc >= 0 and nc < width:
                idx: int = nr * width + nc
                count = count + grid[idx]
            dc = dc + 1
        dr = dr + 1
    return count


def step_life(grid: list[int], width: int, height: int) -> list[int]:
    """Compute one step of Game of Life."""
    new_grid: list[int] = []
    r: int = 0
    while r < height:
        c: int = 0
        while c < width:
            neighbors: int = count_neighbors(grid, width, height, r, c)
            idx: int = r * width + c
            cell: int = grid[idx]
            if cell == 1:
                if neighbors == 2 or neighbors == 3:
                    new_grid.append(1)
                else:
                    new_grid.append(0)
            else:
                if neighbors == 3:
                    new_grid.append(1)
                else:
                    new_grid.append(0)
            c = c + 1
        r = r + 1
    return new_grid


def count_alive(grid: list[int]) -> int:
    """Count total alive cells in grid."""
    total: int = 0
    i: int = 0
    length: int = len(grid)
    while i < length:
        total = total + grid[i]
        i = i + 1
    return total


def test_module() -> int:
    """Test Game of Life operations."""
    passed: int = 0

    blinker: list[int] = [
        0, 0, 0, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 1, 0, 0,
        0, 0, 0, 0, 0,
    ]
    next_gen: list[int] = step_life(blinker, 5, 5)
    if get_cell(next_gen, 5, 2, 1) == 1:
        passed = passed + 1

    if get_cell(next_gen, 5, 2, 3) == 1:
        passed = passed + 1

    if get_cell(next_gen, 5, 1, 2) == 0:
        passed = passed + 1

    if count_alive(next_gen) == 3:
        passed = passed + 1

    block: list[int] = [
        0, 0, 0, 0,
        0, 1, 1, 0,
        0, 1, 1, 0,
        0, 0, 0, 0,
    ]
    block_next: list[int] = step_life(block, 4, 4)
    if count_alive(block_next) == 4:
        passed = passed + 1

    n1: int = count_neighbors(blinker, 5, 5, 2, 2)
    if n1 == 2:
        passed = passed + 1

    n2: int = count_neighbors(blinker, 5, 5, 2, 1)
    if n2 == 1:
        passed = passed + 1

    if count_alive(blinker) == 3:
        passed = passed + 1

    return passed
