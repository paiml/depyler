def nim_can_win(n: int) -> int:
    if n % 4 == 0:
        return 0
    return 1

def nim_optimal_move(n: int) -> int:
    remainder: int = n % 4
    if remainder == 0:
        return 1
    return remainder

def nim_multi_pile(piles: list[int]) -> int:
    xor_sum: int = 0
    i: int = 0
    n: int = len(piles)
    while i < n:
        xor_sum = xor_sum ^ piles[i]
        i = i + 1
    if xor_sum != 0:
        return 1
    return 0

def nim_sprague_grundy(n: int, max_take: int) -> int:
    return n % (max_take + 1)

def play_nim_game(n: int) -> int:
    stones: int = n
    player: int = 1
    while stones > 0:
        take: int = nim_optimal_move(stones)
        stones = stones - take
        if stones == 0:
            return player
        if player == 1:
            player = 2
        else:
            player = 1
    return 0

def nim_sum_piles(piles: list[int]) -> int:
    xor_val: int = 0
    i: int = 0
    n: int = len(piles)
    while i < n:
        xor_val = xor_val ^ piles[i]
        i = i + 1
    return xor_val

def test_module() -> int:
    passed: int = 0
    r1: int = nim_can_win(4)
    if r1 == 0:
        passed = passed + 1
    r2: int = nim_can_win(5)
    if r2 == 1:
        passed = passed + 1
    r3: int = nim_optimal_move(7)
    if r3 == 3:
        passed = passed + 1
    r4: int = nim_multi_pile([3, 4, 5])
    if r4 == 1:
        passed = passed + 1
    r5: int = nim_sprague_grundy(10, 3)
    if r5 == 2:
        passed = passed + 1
    r6: int = nim_sum_piles([1, 2, 3])
    if r6 == 0:
        passed = passed + 1
    return passed

if __name__ == "__main__":
    print(test_module())
