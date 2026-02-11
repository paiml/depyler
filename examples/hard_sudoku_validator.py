"""Validate a sudoku board stored as a flat list of 81 integers."""


def get_sudoku_cell(board: list[int], row: int, col: int) -> int:
    """Get cell value. 0 means empty."""
    return board[row * 9 + col]


def check_row(board: list[int], row: int) -> int:
    """Check if row has no duplicate non-zero values. Returns 1 if valid."""
    seen: list[int] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    col: int = 0
    while col < 9:
        val: int = get_sudoku_cell(board, row, col)
        if val != 0:
            if seen[val] == 1:
                return 0
            seen[val] = 1
        col = col + 1
    return 1


def check_col(board: list[int], col: int) -> int:
    """Check if column has no duplicate non-zero values. Returns 1 if valid."""
    seen: list[int] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    row: int = 0
    while row < 9:
        val: int = get_sudoku_cell(board, row, col)
        if val != 0:
            if seen[val] == 1:
                return 0
            seen[val] = 1
        row = row + 1
    return 1


def check_box(board: list[int], box_row: int, box_col: int) -> int:
    """Check 3x3 box for duplicates. box_row, box_col are 0,3,6. Returns 1 if valid."""
    seen: list[int] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
    r: int = box_row
    while r < box_row + 3:
        c: int = box_col
        while c < box_col + 3:
            val: int = get_sudoku_cell(board, r, c)
            if val != 0:
                if seen[val] == 1:
                    return 0
                seen[val] = 1
            c = c + 1
        r = r + 1
    return 1


def validate_sudoku(board: list[int]) -> int:
    """Full sudoku validation. Returns 1 if valid."""
    r: int = 0
    while r < 9:
        if check_row(board, r) == 0:
            return 0
        r = r + 1
    c: int = 0
    while c < 9:
        if check_col(board, c) == 0:
            return 0
        c = c + 1
    br: int = 0
    while br < 9:
        bc: int = 0
        while bc < 9:
            if check_box(board, br, bc) == 0:
                return 0
            bc = bc + 3
        br = br + 3
    return 1


def count_filled(board: list[int]) -> int:
    """Count filled cells."""
    count: int = 0
    i: int = 0
    while i < 81:
        if board[i] != 0:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test sudoku validator."""
    passed: int = 0

    valid: list[int] = [
        5, 3, 4, 6, 7, 8, 9, 1, 2,
        6, 7, 2, 1, 9, 5, 3, 4, 8,
        1, 9, 8, 3, 4, 2, 5, 6, 7,
        8, 5, 9, 7, 6, 1, 4, 2, 3,
        4, 2, 6, 8, 5, 3, 7, 9, 1,
        7, 1, 3, 9, 2, 4, 8, 5, 6,
        9, 6, 1, 5, 3, 7, 2, 8, 4,
        2, 8, 7, 4, 1, 9, 6, 3, 5,
        3, 4, 5, 2, 8, 6, 1, 7, 9
    ]
    if validate_sudoku(valid) == 1:
        passed = passed + 1

    invalid: list[int] = [
        5, 3, 4, 6, 7, 8, 9, 1, 2,
        6, 7, 2, 1, 9, 5, 3, 4, 8,
        1, 9, 8, 3, 4, 2, 5, 6, 7,
        8, 5, 9, 7, 6, 1, 4, 2, 3,
        4, 2, 6, 8, 5, 3, 7, 9, 1,
        7, 1, 3, 9, 2, 4, 8, 5, 6,
        9, 6, 1, 5, 3, 7, 2, 8, 4,
        2, 8, 7, 4, 1, 9, 6, 3, 5,
        3, 4, 5, 2, 8, 6, 1, 7, 5
    ]
    if validate_sudoku(invalid) == 0:
        passed = passed + 1

    if check_row(valid, 0) == 1:
        passed = passed + 1

    if check_col(valid, 0) == 1:
        passed = passed + 1

    if count_filled(valid) == 81:
        passed = passed + 1

    partial: list[int] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0
    ]
    if validate_sudoku(partial) == 1:
        passed = passed + 1

    return passed
