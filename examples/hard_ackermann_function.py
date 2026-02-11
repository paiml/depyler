"""Ackermann function computed iteratively with explicit stack."""


def ackermann_iter(m: int, n: int) -> int:
    """Compute Ackermann(m, n) iteratively using a stack."""
    stack: list[int] = []
    stack.append(m)
    result: int = n
    while len(stack) > 0:
        top: int = stack[len(stack) - 1]
        stack.pop()
        if top == 0:
            result = result + 1
        elif result == 0:
            stack.append(top - 1)
            result = 1
        else:
            stack.append(top - 1)
            stack.append(top)
            result = result - 1
    return result


def ackermann_zero_n(n: int) -> int:
    """Ackermann(0, n) = n + 1."""
    return ackermann_iter(0, n)


def ackermann_one_n(n: int) -> int:
    """Ackermann(1, n) = n + 2."""
    return ackermann_iter(1, n)


def ackermann_two_n(n: int) -> int:
    """Ackermann(2, n) = 2n + 3."""
    return ackermann_iter(2, n)


def ackermann_three_zero() -> int:
    """Ackermann(3, 0) = 5."""
    return ackermann_iter(3, 0)


def test_module() -> int:
    """Test Ackermann function computations."""
    ok: int = 0
    if ackermann_zero_n(0) == 1:
        ok = ok + 1
    if ackermann_zero_n(5) == 6:
        ok = ok + 1
    if ackermann_one_n(0) == 2:
        ok = ok + 1
    if ackermann_one_n(3) == 5:
        ok = ok + 1
    if ackermann_two_n(0) == 3:
        ok = ok + 1
    if ackermann_two_n(2) == 7:
        ok = ok + 1
    if ackermann_three_zero() == 5:
        ok = ok + 1
    return ok
