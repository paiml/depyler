def make_board() -> list[int]:
    board: list[int] = [0, 0, 0, 0, 0, 0, 0, 0, 0]
    return board

def place_move(board: list[int], pos: int, player: int) -> int:
    v: int = board[pos]
    if v != 0:
        return 0
    board[pos] = player
    return 1

def check_line(board: list[int], a: int, b: int, c: int) -> int:
    va: int = board[a]
    vb: int = board[b]
    vc: int = board[c]
    if va != 0 and va == vb and vb == vc:
        return va
    return 0

def check_winner(board: list[int]) -> int:
    lines: list[list[int]] = [[0,1,2],[3,4,5],[6,7,8],[0,3,6],[1,4,7],[2,5,8],[0,4,8],[2,4,6]]
    i: int = 0
    while i < 8:
        line: list[int] = lines[i]
        a: int = line[0]
        b: int = line[1]
        c: int = line[2]
        w: int = check_line(board, a, b, c)
        if w != 0:
            return w
        i = i + 1
    return 0

def is_full(board: list[int]) -> int:
    i: int = 0
    while i < 9:
        v: int = board[i]
        if v == 0:
            return 0
        i = i + 1
    return 1

def count_moves(board: list[int]) -> int:
    count: int = 0
    i: int = 0
    while i < 9:
        v: int = board[i]
        if v != 0:
            count = count + 1
        i = i + 1
    return count

def empty_positions(board: list[int]) -> list[int]:
    result: list[int] = []
    i: int = 0
    while i < 9:
        v: int = board[i]
        if v == 0:
            result.append(i)
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    b: list[int] = make_board()
    cm: int = count_moves(b)
    if cm == 0:
        passed = passed + 1
    r: int = place_move(b, 0, 1)
    if r == 1:
        passed = passed + 1
    r2: int = place_move(b, 0, 2)
    if r2 == 0:
        passed = passed + 1
    b2: list[int] = [1, 1, 1, 0, 0, 0, 0, 0, 0]
    w: int = check_winner(b2)
    if w == 1:
        passed = passed + 1
    ep: list[int] = empty_positions(b)
    ne: int = len(ep)
    if ne == 8:
        passed = passed + 1
    return passed
