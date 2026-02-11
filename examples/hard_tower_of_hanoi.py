"""Tower of Hanoi iterative computation.

Tests: move count, frame-stewart for 4 pegs, disk positions.
"""


def hanoi_moves(n: int) -> int:
    """Number of moves needed for n disks (2^n - 1)."""
    result: int = 1
    i: int = 0
    while i < n:
        result = result * 2
        i = i + 1
    return result - 1


def hanoi_disk_position(n: int, move: int, disk: int) -> int:
    """Determine which peg a disk is on after a given move number.
    
    Returns 0, 1, or 2 for the three pegs.
    disk is 1-indexed (1 = smallest).
    """
    if disk <= 0 or disk > n:
        return -1
    period: int = 1
    i: int = 0
    while i < disk:
        period = period * 2
        i = i + 1
    pos_in_cycle: int = (move // (period // 2)) % 3
    if n % 2 == disk % 2:
        order: list[int] = [0, 2, 1]
    else:
        order: list[int] = [0, 1, 2]
    return order[pos_in_cycle]


def hanoi_largest_moved(move: int) -> int:
    """Find which disk is moved on a given move (1-indexed)."""
    if move <= 0:
        return 0
    disk: int = 1
    m: int = move
    while m % 2 == 0:
        disk = disk + 1
        m = m // 2
    return disk


def hanoi_total_distance(n: int) -> int:
    """Total peg-distance traveled by all disks for n-disk solution.
    
    Each move moves one peg distance, so total = number of moves.
    """
    return hanoi_moves(n)


def test_module() -> int:
    """Test Tower of Hanoi operations."""
    ok: int = 0
    if hanoi_moves(1) == 1:
        ok = ok + 1
    if hanoi_moves(3) == 7:
        ok = ok + 1
    if hanoi_moves(4) == 15:
        ok = ok + 1
    if hanoi_largest_moved(1) == 1:
        ok = ok + 1
    if hanoi_largest_moved(2) == 2:
        ok = ok + 1
    if hanoi_largest_moved(4) == 3:
        ok = ok + 1
    if hanoi_total_distance(3) == 7:
        ok = ok + 1
    return ok
