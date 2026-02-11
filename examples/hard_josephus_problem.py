"""Josephus problem simulation.

Tests: josephus position, survivor, generalized josephus.
"""


def josephus(n: int, k: int) -> int:
    """Find the position of the last survivor (0-indexed)."""
    if n == 1:
        return 0
    result: int = 0
    i: int = 2
    while i <= n:
        result = (result + k) % i
        i = i + 1
    return result


def josephus_one_indexed(n: int, k: int) -> int:
    """Find survivor position (1-indexed)."""
    return josephus(n, k) + 1


def josephus_sequence_last(n: int, k: int) -> int:
    """Return the last element eliminated (0-indexed)."""
    return josephus(n, k)


def josephus_first_eliminated(n: int, k: int) -> int:
    """Find the first person eliminated (0-indexed)."""
    if n == 0:
        return -1
    return (k - 1) % n


def josephus_with_start(n: int, k: int, start: int) -> int:
    """Josephus with custom starting position (0-indexed result)."""
    base: int = josephus(n, k)
    return (base + start) % n


def test_module() -> int:
    """Test Josephus problem variants."""
    ok: int = 0
    if josephus(1, 3) == 0:
        ok = ok + 1
    if josephus(5, 2) == 2:
        ok = ok + 1
    if josephus(7, 3) == 3:
        ok = ok + 1
    if josephus_one_indexed(5, 2) == 3:
        ok = ok + 1
    if josephus_first_eliminated(5, 2) == 1:
        ok = ok + 1
    if josephus_first_eliminated(7, 3) == 2:
        ok = ok + 1
    if josephus_with_start(5, 2, 0) == 2:
        ok = ok + 1
    return ok
