def make_c4_board(rows: int, cols: int) -> list[list[int]]:
    board: list[list[int]] = []
    r: int = 0
    while r < rows:
        row: list[int] = []
        c: int = 0
        while c < cols:
            row.append(0)
            c = c + 1
        board.append(row)
        r = r + 1
    return board

def drop_piece(board: list[list[int]], col: int, player: int, rows: int) -> int:
    r: int = rows - 1
    while r >= 0:
        row: list[int] = board[r]
        v: int = row[col]
        if v == 0:
            row[col] = player
            return r
        r = r - 1
    return 0 - 1

def check_horizontal(board: list[list[int]], rows: int, cols: int) -> int:
    r: int = 0
    while r < rows:
        c: int = 0
        while c <= cols - 4:
            row: list[int] = board[r]
            v0: int = row[c]
            v1: int = row[c + 1]
            v2: int = row[c + 2]
            v3: int = row[c + 3]
            if v0 != 0 and v0 == v1 and v1 == v2 and v2 == v3:
                return v0
            c = c + 1
        r = r + 1
    return 0

def check_vertical(board: list[list[int]], rows: int, cols: int) -> int:
    c: int = 0
    while c < cols:
        r: int = 0
        while r <= rows - 4:
            r0: list[int] = board[r]
            r1: list[int] = board[r + 1]
            r2: list[int] = board[r + 2]
            r3: list[int] = board[r + 3]
            v0: int = r0[c]
            v1: int = r1[c]
            v2: int = r2[c]
            v3: int = r3[c]
            if v0 != 0 and v0 == v1 and v1 == v2 and v2 == v3:
                return v0
            r = r + 1
        c = c + 1
    return 0

def column_full(board: list[list[int]], col: int) -> int:
    top_row: list[int] = board[0]
    v: int = top_row[col]
    if v != 0:
        return 1
    return 0

def test_module() -> int:
    passed: int = 0
    b: list[list[int]] = make_c4_board(6, 7)
    nb: int = len(b)
    if nb == 6:
        passed = passed + 1
    r: int = drop_piece(b, 0, 1, 6)
    if r == 5:
        passed = passed + 1
    r2: int = drop_piece(b, 0, 2, 6)
    if r2 == 4:
        passed = passed + 1
    cf: int = column_full(b, 0)
    if cf == 0:
        passed = passed + 1
    h: int = check_horizontal(b, 6, 7)
    if h == 0:
        passed = passed + 1
    return passed
