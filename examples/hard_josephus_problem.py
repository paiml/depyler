"""Josephus problem: find survivor position."""


def josephus(n: int, k: int) -> int:
    """Find 0-indexed position of survivor in Josephus problem."""
    if n == 1:
        return 0
    result: int = 0
    i: int = 2
    while i <= n:
        result = (result + k) % i
        i = i + 1
    return result


def josephus_one_indexed(n: int, k: int) -> int:
    """Find 1-indexed survivor position."""
    return josephus(n, k) + 1


def josephus_sequence(n: int, k: int) -> list[int]:
    """Return elimination order (0-indexed)."""
    alive: list[int] = []
    i: int = 0
    while i < n:
        alive.append(i)
        i = i + 1
    order: list[int] = []
    idx: int = 0
    while len(alive) > 0:
        idx = (idx + k - 1) % len(alive)
        val: int = alive[idx]
        order.append(val)
        j: int = idx
        while j < len(alive) - 1:
            alive[j] = alive[j + 1]
            j = j + 1
        alive.pop()
        if len(alive) > 0:
            idx = idx % len(alive)
    return order


def last_eliminated(n: int, k: int) -> int:
    """Return last person eliminated (== survivor)."""
    seq: list[int] = josephus_sequence(n, k)
    return seq[len(seq) - 1]


def test_module() -> int:
    """Test Josephus problem solutions."""
    ok: int = 0
    if josephus(1, 1) == 0:
        ok = ok + 1
    if josephus(5, 2) == 2:
        ok = ok + 1
    if josephus(7, 3) == 3:
        ok = ok + 1
    if josephus_one_indexed(5, 2) == 3:
        ok = ok + 1
    seq: list[int] = josephus_sequence(5, 2)
    if len(seq) == 5:
        ok = ok + 1
    if last_eliminated(5, 2) == josephus(5, 2):
        ok = ok + 1
    if josephus(3, 2) == 2:
        ok = ok + 1
    return ok
