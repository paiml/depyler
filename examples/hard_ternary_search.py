"""Ternary search on unimodal functions and arrays."""


def ternary_search_max(arr: list[int]) -> int:
    """Find index of maximum in a unimodal array using ternary search."""
    lo: int = 0
    hi: int = len(arr) - 1
    while hi - lo > 2:
        m1: int = lo + (hi - lo) // 3
        m2: int = hi - (hi - lo) // 3
        if arr[m1] < arr[m2]:
            lo = m1 + 1
        else:
            hi = m2 - 1
    best_idx: int = lo
    i: int = lo + 1
    while i <= hi:
        if arr[i] > arr[best_idx]:
            best_idx = i
        i = i + 1
    return best_idx


def ternary_search_min(arr: list[int]) -> int:
    """Find index of minimum in a valley-shaped array."""
    lo: int = 0
    hi: int = len(arr) - 1
    while hi - lo > 2:
        m1: int = lo + (hi - lo) // 3
        m2: int = hi - (hi - lo) // 3
        if arr[m1] > arr[m2]:
            lo = m1 + 1
        else:
            hi = m2 - 1
    best_idx: int = lo
    i: int = lo + 1
    while i <= hi:
        if arr[i] < arr[best_idx]:
            best_idx = i
        i = i + 1
    return best_idx


def eval_quadratic(a: int, b: int, c: int, x: int) -> int:
    """Evaluate ax^2 + bx + c."""
    return a * x * x + b * x + c


def find_vertex_x(a: int, b: int, lo: int, hi: int) -> int:
    """Find x that minimizes/maximizes ax^2+bx+c in integer range [lo,hi]."""
    while hi - lo > 2:
        m1: int = lo + (hi - lo) // 3
        m2: int = hi - (hi - lo) // 3
        v1: int = a * m1 * m1 + b * m1
        v2: int = a * m2 * m2 + b * m2
        if a > 0:
            if v1 < v2:
                hi = m2 - 1
            else:
                lo = m1 + 1
        else:
            if v1 > v2:
                hi = m2 - 1
            else:
                lo = m1 + 1
    best: int = lo
    i: int = lo + 1
    while i <= hi:
        bv: int = a * best * best + b * best
        iv: int = a * i * i + b * i
        if a > 0 and iv < bv:
            best = i
        if a < 0 and iv > bv:
            best = i
        i = i + 1
    return best


def test_module() -> int:
    passed: int = 0

    unimodal: list[int] = [1, 3, 7, 12, 15, 13, 8, 4, 2]
    if ternary_search_max(unimodal) == 4:
        passed = passed + 1

    valley: list[int] = [10, 7, 4, 2, 1, 3, 6, 9]
    if ternary_search_min(valley) == 4:
        passed = passed + 1

    if eval_quadratic(1, -4, 4, 2) == 0:
        passed = passed + 1

    if eval_quadratic(1, 0, 0, 3) == 9:
        passed = passed + 1

    vx: int = find_vertex_x(1, -6, 0, 10)
    if vx == 3:
        passed = passed + 1

    peak: list[int] = [1, 5, 10, 5, 1]
    if ternary_search_max(peak) == 2:
        passed = passed + 1

    valley2: list[int] = [20, 10, 5, 2, 5, 10, 20]
    if ternary_search_min(valley2) == 3:
        passed = passed + 1

    return passed
