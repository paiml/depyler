def hanoi_move_count(n: int) -> int:
    if n <= 0:
        return 0
    return (1 << n) - 1

def hanoi_moves(n: int, src: int, dst: int, aux: int) -> list[list[int]]:
    moves: list[list[int]] = []
    hanoi_helper(n, src, dst, aux, moves)
    return moves

def hanoi_helper(n: int, src: int, dst: int, aux: int, moves: list[list[int]]) -> int:
    if n == 0:
        return 0
    hanoi_helper(n - 1, src, aux, dst, moves)
    moves.append([src, dst])
    hanoi_helper(n - 1, aux, dst, src, moves)
    return 0

def hanoi_iterative_count(n: int) -> int:
    total: int = 1
    i: int = 0
    while i < n:
        total = total * 2
        i = i + 1
    return total - 1

def frame_stewart(n: int, pegs: int) -> int:
    if pegs < 3 or n <= 0:
        return 0
    if pegs == 3:
        return hanoi_move_count(n)
    if n == 1:
        return 1
    best: int = 999999
    k: int = 1
    while k < n:
        sub: int = 2 * frame_stewart(k, pegs) + frame_stewart(n - k, pegs - 1)
        if sub < best:
            best = sub
        k = k + 1
    return best

def test_module() -> int:
    passed: int = 0
    r1: int = hanoi_move_count(3)
    if r1 == 7:
        passed = passed + 1
    r2: int = hanoi_move_count(4)
    if r2 == 15:
        passed = passed + 1
    m: list[list[int]] = hanoi_moves(3, 1, 3, 2)
    nm: int = len(m)
    if nm == 7:
        passed = passed + 1
    r3: int = hanoi_iterative_count(3)
    if r3 == 7:
        passed = passed + 1
    r4: int = frame_stewart(3, 4)
    if r4 == 5:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
