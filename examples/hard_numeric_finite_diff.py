"""Finite difference approximations for derivatives (integer-scaled)."""


def forward_diff(vals: list[int], h: int) -> list[int]:
    """Forward difference derivative: f'(i) ~ (f(i+1)-f(i))/h.
    Returns derivatives * h (to avoid division)."""
    n: int = len(vals)
    result: list[int] = []
    i: int = 0
    while i < n - 1:
        d: int = vals[i + 1] - vals[i]
        result.append(d)
        i = i + 1
    return result


def backward_diff(vals: list[int], h: int) -> list[int]:
    """Backward difference: f'(i) ~ (f(i)-f(i-1))/h.
    Returns derivatives * h."""
    n: int = len(vals)
    result: list[int] = []
    i: int = 1
    while i < n:
        d: int = vals[i] - vals[i - 1]
        result.append(d)
        i = i + 1
    return result


def central_diff(vals: list[int], h: int) -> list[int]:
    """Central difference: f'(i) ~ (f(i+1)-f(i-1))/(2h).
    Returns derivatives * 2h."""
    n: int = len(vals)
    result: list[int] = []
    i: int = 1
    while i < n - 1:
        d: int = vals[i + 1] - vals[i - 1]
        result.append(d)
        i = i + 1
    return result


def second_derivative(vals: list[int], h: int) -> list[int]:
    """Second derivative: f''(i) ~ (f(i+1)-2*f(i)+f(i-1))/h^2.
    Returns derivatives * h^2."""
    n: int = len(vals)
    result: list[int] = []
    i: int = 1
    while i < n - 1:
        d: int = vals[i + 1] - 2 * vals[i] + vals[i - 1]
        result.append(d)
        i = i + 1
    return result


def laplacian_1d(vals: list[int]) -> list[int]:
    """1D discrete Laplacian (same as second derivative with h=1)."""
    return second_derivative(vals, 1)


def test_module() -> int:
    """Test finite difference functions."""
    ok: int = 0
    vals: list[int] = [0, 1, 4, 9, 16]
    fd: list[int] = forward_diff(vals, 1)
    if fd[0] == 1:
        ok = ok + 1
    if fd[1] == 3:
        ok = ok + 1
    cd: list[int] = central_diff(vals, 1)
    if cd[0] == 4:
        ok = ok + 1
    sd: list[int] = second_derivative(vals, 1)
    if sd[0] == 2:
        ok = ok + 1
    if sd[1] == 2:
        ok = ok + 1
    return ok
