"""Tower of Hanoi iterative solution."""


def hanoi_move_count(n: int) -> int:
    """Compute total moves needed for n disks: 2^n - 1."""
    result: int = 1
    i: int = 0
    while i < n:
        result = result * 2
        i = i + 1
    return result - 1


def hanoi_iterative(n: int) -> list[int]:
    """Solve Tower of Hanoi iteratively. Returns list of [from, to] pairs flattened."""
    total_moves: int = hanoi_move_count(n)
    moves: list[int] = []
    pegs: list[int] = []
    i: int = 0
    while i < n:
        pegs.append(0)
        i = i + 1
    move_num: int = 1
    while move_num <= total_moves:
        disk: int = 0
        temp_move: int = move_num
        while temp_move % 2 == 0:
            disk = disk + 1
            temp_move = temp_move // 2
        if disk < n:
            from_peg: int = pegs[disk]
            if disk % 2 == 0:
                if n % 2 == 0:
                    to_peg: int = (from_peg + 1) % 3
                else:
                    to_peg = (from_peg + 2) % 3
            else:
                if n % 2 == 0:
                    to_peg = (from_peg + 2) % 3
                else:
                    to_peg = (from_peg + 1) % 3
            moves.append(from_peg)
            moves.append(to_peg)
            pegs[disk] = to_peg
        move_num = move_num + 1
    return moves


def verify_hanoi_solution(n: int, moves: list[int]) -> int:
    """Verify a Hanoi solution is valid. Returns 1 if valid, 0 if not."""
    stacks: list[int] = []
    i: int = 0
    while i < 3 * n:
        stacks.append(0)
        i = i + 1
    sizes: list[int] = [0, 0, 0]
    disk: int = n
    while disk > 0:
        idx: int = sizes[0]
        stacks[idx] = disk
        sizes[0] = sizes[0] + 1
        disk = disk - 1
    m: int = 0
    move_count: int = len(moves) // 2
    while m < move_count:
        from_peg: int = moves[m * 2]
        to_peg: int = moves[m * 2 + 1]
        if sizes[from_peg] == 0:
            return 0
        sizes[from_peg] = sizes[from_peg] - 1
        from_idx: int = from_peg * n + sizes[from_peg]
        moving_disk: int = stacks[from_idx]
        to_idx: int = to_peg * n + sizes[to_peg]
        if sizes[to_peg] > 0:
            top_idx: int = to_peg * n + sizes[to_peg] - 1
            if stacks[top_idx] < moving_disk:
                return 0
        stacks[to_idx] = moving_disk
        sizes[to_peg] = sizes[to_peg] + 1
        m = m + 1
    if sizes[2] == n:
        return 1
    return 0


def test_module() -> int:
    """Test Tower of Hanoi operations."""
    passed: int = 0

    if hanoi_move_count(1) == 1:
        passed = passed + 1

    if hanoi_move_count(3) == 7:
        passed = passed + 1

    if hanoi_move_count(4) == 15:
        passed = passed + 1

    moves1: list[int] = hanoi_iterative(1)
    if len(moves1) == 2:
        passed = passed + 1

    moves3: list[int] = hanoi_iterative(3)
    if len(moves3) == 14:
        passed = passed + 1

    if verify_hanoi_solution(1, hanoi_iterative(1)) == 1:
        passed = passed + 1

    if verify_hanoi_solution(2, hanoi_iterative(2)) == 1:
        passed = passed + 1

    if verify_hanoi_solution(3, hanoi_iterative(3)) == 1:
        passed = passed + 1

    return passed
