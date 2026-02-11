"""Minimax evaluation for tic-tac-toe positions."""


def check_winner(board: list[int]) -> int:
    """Check for winner. Returns 1 for X, 2 for O, 0 for none.
    Board is 9 cells: 0=empty, 1=X, 2=O.
    """
    lines: list[int] = [
        0, 1, 2, 3, 4, 5, 6, 7, 8,
        0, 3, 6, 1, 4, 7, 2, 5, 8,
        0, 4, 8, 2, 4, 6
    ]
    i: int = 0
    while i < 24:
        a: int = lines[i]
        b: int = lines[i + 1]
        c: int = lines[i + 2]
        if board[a] != 0 and board[a] == board[b] and board[b] == board[c]:
            return board[a]
        i = i + 3
    return 0


def count_empty(board: list[int]) -> int:
    """Count empty cells."""
    count: int = 0
    i: int = 0
    while i < 9:
        if board[i] == 0:
            count = count + 1
        i = i + 1
    return count


def minimax(board: list[int], is_maximizing: int) -> int:
    """Minimax evaluation. X=maximizer(1), O=minimizer(2).
    Returns score: 10 for X win, -10 for O win, 0 for draw.
    """
    winner: int = check_winner(board)
    if winner == 1:
        return 10
    if winner == 2:
        return 0 - 10
    if count_empty(board) == 0:
        return 0
    if is_maximizing == 1:
        best: int = 0 - 100
        i: int = 0
        while i < 9:
            if board[i] == 0:
                board[i] = 1
                score: int = minimax(board, 0)
                board[i] = 0
                if score > best:
                    best = score
            i = i + 1
        return best
    else:
        best2: int = 100
        i2: int = 0
        while i2 < 9:
            if board[i2] == 0:
                board[i2] = 2
                score2: int = minimax(board, 1)
                board[i2] = 0
                if score2 < best2:
                    best2 = score2
            i2 = i2 + 1
        return best2


def find_best_move(board: list[int], player: int) -> int:
    """Find best move index for given player (1=X, 2=O)."""
    best_move: int = 0 - 1
    if player == 1:
        best_score: int = 0 - 100
        i: int = 0
        while i < 9:
            if board[i] == 0:
                board[i] = 1
                score: int = minimax(board, 0)
                board[i] = 0
                if score > best_score:
                    best_score = score
                    best_move = i
            i = i + 1
    else:
        best_score2: int = 100
        j: int = 0
        while j < 9:
            if board[j] == 0:
                board[j] = 2
                score2: int = minimax(board, 1)
                board[j] = 0
                if score2 < best_score2:
                    best_score2 = score2
                    best_move = j
            j = j + 1
    return best_move


def test_module() -> int:
    """Test minimax tic-tac-toe."""
    passed: int = 0

    b1: list[int] = [1, 1, 1, 0, 0, 0, 0, 0, 0]
    if check_winner(b1) == 1:
        passed = passed + 1

    b2: list[int] = [2, 0, 0, 2, 0, 0, 2, 0, 0]
    if check_winner(b2) == 2:
        passed = passed + 1

    b3: list[int] = [1, 2, 1, 1, 2, 2, 2, 1, 1]
    if check_winner(b3) == 0:
        passed = passed + 1

    b4: list[int] = [1, 2, 0, 0, 1, 0, 0, 0, 0]
    mv: int = find_best_move(b4, 1)
    if mv == 8:
        passed = passed + 1

    b5: list[int] = [0, 0, 0, 0, 0, 0, 0, 0, 0]
    if count_empty(b5) == 9:
        passed = passed + 1

    if count_empty(b3) == 0:
        passed = passed + 1

    return passed
