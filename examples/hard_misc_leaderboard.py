def create_leaderboard() -> list[list[int]]:
    board: list[list[int]] = []
    return board

def add_score(board: list[list[int]], player_id: int, score: int) -> int:
    n: int = len(board)
    found: int = 0 - 1
    i: int = 0
    while i < n:
        entry: list[int] = board[i]
        pid: int = entry[0]
        if pid == player_id:
            found = i
        i = i + 1
    if found >= 0:
        entry2: list[int] = board[found]
        old_score: int = entry2[1]
        if score > old_score:
            entry2[1] = score
    else:
        board.append([player_id, score])
    return len(board)

def get_rank(board: list[list[int]], player_id: int) -> int:
    n: int = len(board)
    player_score: int = 0 - 1
    i: int = 0
    while i < n:
        entry: list[int] = board[i]
        pid: int = entry[0]
        if pid == player_id:
            player_score = entry[1]
        i = i + 1
    if player_score < 0:
        return 0 - 1
    rank: int = 1
    j: int = 0
    while j < n:
        entry2: list[int] = board[j]
        s: int = entry2[1]
        if s > player_score:
            rank = rank + 1
        j = j + 1
    return rank

def top_k(board: list[list[int]], k_val: int) -> list[int]:
    n: int = len(board)
    indices: list[int] = []
    i: int = 0
    while i < n:
        indices.append(i)
        i = i + 1
    j: int = 0
    while j < n - 1:
        m: int = j + 1
        while m < n:
            ij: int = indices[j]
            im: int = indices[m]
            ej: list[int] = board[ij]
            em: list[int] = board[im]
            sj: int = ej[1]
            sm: int = em[1]
            if sm > sj:
                indices[j] = im
                indices[m] = ij
            m = m + 1
        j = j + 1
    result: list[int] = []
    cnt: int = 0
    while cnt < k_val and cnt < n:
        idx: int = indices[cnt]
        entry: list[int] = board[idx]
        result.append(entry[0])
        cnt = cnt + 1
    return result

def board_size(board: list[list[int]]) -> int:
    return len(board)

def test_module() -> int:
    passed: int = 0
    b: list[list[int]] = create_leaderboard()
    add_score(b, 1, 100)
    add_score(b, 2, 200)
    add_score(b, 3, 150)
    bs: int = board_size(b)
    if bs == 3:
        passed = passed + 1
    r: int = get_rank(b, 2)
    if r == 1:
        passed = passed + 1
    r2: int = get_rank(b, 1)
    if r2 == 3:
        passed = passed + 1
    tk: list[int] = top_k(b, 2)
    tk0: int = tk[0]
    if tk0 == 2:
        passed = passed + 1
    add_score(b, 1, 300)
    r3: int = get_rank(b, 1)
    if r3 == 1:
        passed = passed + 1
    return passed
