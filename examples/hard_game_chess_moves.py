def in_bounds(r: int, c: int) -> int:
    if r >= 0 and r < 8 and c >= 0 and c < 8:
        return 1
    return 0

def rook_moves(r: int, c: int) -> list[list[int]]:
    moves: list[list[int]] = []
    i: int = 0
    while i < 8:
        if i != r:
            moves.append([i, c])
        if i != c:
            moves.append([r, i])
        i = i + 1
    return moves

def bishop_moves(r: int, c: int) -> list[list[int]]:
    moves: list[list[int]] = []
    d: int = 1
    while d < 8:
        nr: int = r + d
        nc: int = c + d
        ib: int = in_bounds(nr, nc)
        if ib == 1:
            moves.append([nr, nc])
        nr2: int = r + d
        nc2: int = c - d
        ib2: int = in_bounds(nr2, nc2)
        if ib2 == 1:
            moves.append([nr2, nc2])
        nr3: int = r - d
        nc3: int = c + d
        ib3: int = in_bounds(nr3, nc3)
        if ib3 == 1:
            moves.append([nr3, nc3])
        nr4: int = r - d
        nc4: int = c - d
        ib4: int = in_bounds(nr4, nc4)
        if ib4 == 1:
            moves.append([nr4, nc4])
        d = d + 1
    return moves

def knight_moves(r: int, c: int) -> list[list[int]]:
    moves: list[list[int]] = []
    offsets: list[list[int]] = [[0-2,0-1],[0-2,1],[0-1,0-2],[0-1,2],[1,0-2],[1,2],[2,0-1],[2,1]]
    i: int = 0
    while i < 8:
        off: list[int] = offsets[i]
        nr: int = r + off[0]
        nc: int = c + off[1]
        ib: int = in_bounds(nr, nc)
        if ib == 1:
            moves.append([nr, nc])
        i = i + 1
    return moves

def king_moves(r: int, c: int) -> list[list[int]]:
    moves: list[list[int]] = []
    dr: int = 0 - 1
    while dr <= 1:
        dc: int = 0 - 1
        while dc <= 1:
            if dr != 0 or dc != 0:
                nr: int = r + dr
                nc: int = c + dc
                ib: int = in_bounds(nr, nc)
                if ib == 1:
                    moves.append([nr, nc])
            dc = dc + 1
        dr = dr + 1
    return moves

def test_module() -> int:
    passed: int = 0
    ib: int = in_bounds(0, 0)
    if ib == 1:
        passed = passed + 1
    ib2: int = in_bounds(8, 0)
    if ib2 == 0:
        passed = passed + 1
    rm: list[list[int]] = rook_moves(0, 0)
    nrm: int = len(rm)
    if nrm == 14:
        passed = passed + 1
    km: list[list[int]] = knight_moves(4, 4)
    nkm: int = len(km)
    if nkm == 8:
        passed = passed + 1
    kgm: list[list[int]] = king_moves(4, 4)
    nkg: int = len(kgm)
    if nkg == 8:
        passed = passed + 1
    return passed
