"""Spiral number generation.

Generates numbers in spiral patterns using flat arrays
with computed offsets for 2D grid simulation.
"""


def spiral_fill(size: int) -> list[int]:
    """Fill a flat array with numbers in clockwise spiral order.

    The flat array represents a size x size grid.
    """
    total: int = size * size
    grid: list[int] = []
    i: int = 0
    while i < total:
        grid.append(0)
        i = i + 1

    num: int = 1
    top: int = 0
    bottom: int = size - 1
    left: int = 0
    right: int = size - 1

    while num <= total:
        col: int = left
        while col <= right and num <= total:
            idx: int = top * size + col
            grid[idx] = num
            num = num + 1
            col = col + 1
        top = top + 1

        row: int = top
        while row <= bottom and num <= total:
            idx2: int = row * size + right
            grid[idx2] = num
            num = num + 1
            row = row + 1
        right = right - 1

        col2: int = right
        while col2 >= left and num <= total:
            idx3: int = bottom * size + col2
            grid[idx3] = num
            num = num + 1
            col2 = col2 - 1
        bottom = bottom - 1

        row2: int = bottom
        while row2 >= top and num <= total:
            idx4: int = row2 * size + left
            grid[idx4] = num
            num = num + 1
            row2 = row2 - 1
        left = left + 1

    return grid


def spiral_sum_ring(grid: list[int], size: int, ring: int) -> int:
    """Sum all elements in a given ring of the spiral grid."""
    total: int = 0
    col: int = ring
    while col < size - ring:
        idx: int = ring * size + col
        total = total + grid[idx]
        col = col + 1
    row: int = ring + 1
    while row < size - ring:
        idx2: int = row * size + (size - ring - 1)
        total = total + grid[idx2]
        row = row + 1
    col2: int = size - ring - 2
    while col2 >= ring:
        idx3: int = (size - ring - 1) * size + col2
        total = total + grid[idx3]
        col2 = col2 - 1
    row2: int = size - ring - 2
    while row2 > ring:
        idx4: int = row2 * size + ring
        total = total + grid[idx4]
        row2 = row2 - 1
    return total


def spiral_corner_sum(grid: list[int], size: int) -> int:
    """Sum the four corner elements of the grid."""
    tl: int = grid[0]
    last_col: int = size - 1
    tr: int = grid[last_col]
    last_row_start: int = (size - 1) * size
    bl: int = grid[last_row_start]
    br_idx: int = last_row_start + last_col
    br: int = grid[br_idx]
    total: int = tl + tr + bl + br
    return total


def test_module() -> int:
    """Test spiral number generation."""
    ok: int = 0

    tmp_grid: list[int] = spiral_fill(3)
    if tmp_grid[0] == 1 and tmp_grid[1] == 2 and tmp_grid[2] == 3:
        ok = ok + 1

    if tmp_grid[4] == 9:
        ok = ok + 1

    corners: int = spiral_corner_sum(tmp_grid, 3)
    if corners == 1 + 3 + 7 + 9:
        ok = ok + 1

    ring_sum: int = spiral_sum_ring(tmp_grid, 3, 0)
    if ring_sum == 1 + 2 + 3 + 6 + 9 + 8 + 7 + 4:
        ok = ok + 1

    return ok
