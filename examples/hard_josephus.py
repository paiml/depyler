"""Josephus problem simulation."""


def josephus(n: int, step: int) -> int:
    """Find the position of the last person standing (0-indexed)."""
    if n <= 0:
        return 0
    result: int = 0
    i: int = 2
    while i <= n:
        result = (result + step) % i
        i = i + 1
    return result


def josephus_sequence(n: int, step: int) -> list[int]:
    """Return the elimination order in the Josephus problem."""
    circle: list[int] = []
    i: int = 0
    while i < n:
        circle.append(i)
        i = i + 1
    order: list[int] = []
    idx: int = 0
    while len(circle) > 0:
        remaining: int = len(circle)
        idx = (idx + step - 1) % remaining
        eliminated: int = circle[idx]
        order.append(eliminated)
        new_circle: list[int] = []
        j: int = 0
        while j < remaining:
            if j != idx:
                new_circle.append(circle[j])
            j = j + 1
        circle = new_circle
        if idx >= len(circle) and len(circle) > 0:
            idx = 0
    return order


def josephus_survivor_1indexed(n: int, step: int) -> int:
    """Find survivor position (1-indexed) for convenience."""
    return josephus(n, step) + 1


def test_module() -> int:
    """Test Josephus problem operations."""
    passed: int = 0

    if josephus(1, 1) == 0:
        passed = passed + 1

    if josephus(5, 2) == 2:
        passed = passed + 1

    if josephus(7, 3) == 3:
        passed = passed + 1

    if josephus_survivor_1indexed(5, 2) == 3:
        passed = passed + 1

    seq: list[int] = josephus_sequence(5, 2)
    if len(seq) == 5:
        passed = passed + 1

    if seq[0] == 1:
        passed = passed + 1

    last_idx: int = len(seq) - 1
    if seq[last_idx] == 2:
        passed = passed + 1

    if josephus(6, 1) == 5:
        passed = passed + 1

    return passed
