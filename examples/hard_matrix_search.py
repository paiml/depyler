"""Search operations in sorted flat matrices."""


def search_sorted_matrix(mat: list[int], rows: int, cols: int, target: int) -> int:
    """Search in row-and-column sorted flat matrix. Return 1 if found, else 0.
    Starts from top-right corner."""
    row: int = 0
    col: int = cols - 1
    while row < rows and col >= 0:
        pos: int = row * cols + col
        val: int = mat[pos]
        if val == target:
            return 1
        if val < target:
            row = row + 1
        else:
            col = col - 1
    return 0


def find_position(mat: list[int], rows: int, cols: int, target: int) -> list[int]:
    """Find position [row, col] of target in sorted matrix. Returns [-1, -1] if not found."""
    row: int = 0
    col: int = cols - 1
    while row < rows and col >= 0:
        pos: int = row * cols + col
        val: int = mat[pos]
        if val == target:
            result: list[int] = [row, col]
            return result
        if val < target:
            row = row + 1
        else:
            col = col - 1
    not_found: list[int] = [-1, -1]
    return not_found


def count_negatives(mat: list[int], rows: int, cols: int) -> int:
    """Count negative numbers in a sorted matrix (rows sorted descending)."""
    count: int = 0
    row: int = 0
    while row < rows:
        col: int = 0
        while col < cols:
            pos: int = row * cols + col
            if mat[pos] < 0:
                count = count + 1
            col = col + 1
        row = row + 1
    return count


def test_module() -> int:
    passed: int = 0

    mat: list[int] = [1, 4, 7, 10, 2, 5, 8, 11, 3, 6, 9, 12]
    if search_sorted_matrix(mat, 3, 4, 5) == 1:
        passed = passed + 1
    if search_sorted_matrix(mat, 3, 4, 13) == 0:
        passed = passed + 1

    pos: list[int] = find_position(mat, 3, 4, 8)
    if pos[0] == 1:
        passed = passed + 1
    if pos[1] == 2:
        passed = passed + 1

    missing: list[int] = find_position(mat, 3, 4, 99)
    if missing[0] == -1:
        passed = passed + 1

    neg_mat: list[int] = [3, 2, -1, -4, 1, 0, -2, -5]
    if count_negatives(neg_mat, 2, 4) == 3:
        passed = passed + 1

    if search_sorted_matrix(mat, 3, 4, 1) == 1:
        passed = passed + 1

    return passed
