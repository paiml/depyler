"""Power iteration for dominant eigenvalue estimation (integer-scaled)."""


def mat_vec_mult(m: list[int], v: list[int], n: int) -> list[int]:
    """Multiply n x n matrix by n-vector."""
    result: list[int] = []
    i: int = 0
    while i < n:
        s: int = 0
        j: int = 0
        while j < n:
            idx: int = i * n + j
            s = s + m[idx] * v[j]
            j = j + 1
        result.append(s)
        i = i + 1
    return result


def vec_max_abs(v: list[int]) -> int:
    """Return element with largest absolute value."""
    n: int = len(v)
    best: int = 0
    best_abs: int = 0
    i: int = 0
    while i < n:
        val: int = v[i]
        av: int = val
        if val < 0:
            av = 0 - val
        if av > best_abs:
            best_abs = av
            best = val
        i = i + 1
    return best


def power_iteration(m: list[int], n: int, iters: int) -> int:
    """Power iteration returning dominant eigenvalue estimate."""
    v: list[int] = []
    i: int = 0
    while i < n:
        v.append(1)
        i = i + 1
    step: int = 0
    eigenval: int = 0
    while step < iters:
        w: list[int] = mat_vec_mult(m, v, n)
        mx: int = vec_max_abs(w)
        if mx == 0:
            return 0
        eigenval = mx
        v = w
        step = step + 1
    return eigenval


def rayleigh_quotient(m: list[int], v: list[int], n: int) -> int:
    """Rayleigh quotient: v^T M v / (v^T v). Integer division."""
    mv: list[int] = mat_vec_mult(m, v, n)
    num: int = 0
    den: int = 0
    i: int = 0
    while i < n:
        num = num + v[i] * mv[i]
        den = den + v[i] * v[i]
        i = i + 1
    if den == 0:
        return 0
    return num // den


def test_module() -> int:
    """Test eigenvalue estimation."""
    ok: int = 0
    m: list[int] = [2, 0, 0, 1]
    v: list[int] = [1, 1]
    mv: list[int] = mat_vec_mult(m, v, 2)
    if mv[0] == 2:
        ok = ok + 1
    if mv[1] == 1:
        ok = ok + 1
    v2: list[int] = [1, 0]
    rq: int = rayleigh_quotient(m, v2, 2)
    if rq == 2:
        ok = ok + 1
    diag: list[int] = [5, 0, 0, 3]
    e: int = power_iteration(diag, 2, 5)
    if e > 0:
        ok = ok + 1
    if vec_max_abs([0 - 7, 3, 5]) == 0 - 7:
        ok = ok + 1
    return ok
