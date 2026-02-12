"""Numerical methods: Continued fractions and rational approximation.

Tests: iterative convergent computation, fraction arithmetic,
best rational approximation, partial quotient extraction.
"""

from typing import List, Tuple


def continued_fraction_sqrt2(terms: int) -> Tuple[int, int]:
    """Compute convergent of sqrt(2) = [1; 2, 2, 2, ...] as p/q."""
    if terms <= 0:
        return (1, 1)
    p_prev: int = 1
    q_prev: int = 0
    p_curr: int = 1
    q_curr: int = 1
    i: int = 0
    while i < terms:
        a: int = 2
        p_new: int = a * p_curr + p_prev
        q_new: int = a * q_curr + q_prev
        p_prev = p_curr
        q_prev = q_curr
        p_curr = p_new
        q_curr = q_new
        i += 1
    return (p_curr, q_curr)


def continued_fraction_golden(terms: int) -> Tuple[int, int]:
    """Compute convergent of golden ratio = [1; 1, 1, 1, ...] as p/q."""
    if terms <= 0:
        return (1, 1)
    p_prev: int = 1
    q_prev: int = 0
    p_curr: int = 1
    q_curr: int = 1
    i: int = 0
    while i < terms:
        p_new: int = p_curr + p_prev
        q_new: int = q_curr + q_prev
        p_prev = p_curr
        q_prev = q_curr
        p_curr = p_new
        q_curr = q_new
        i += 1
    return (p_curr, q_curr)


def fraction_add(p1: int, q1: int, p2: int, q2: int) -> Tuple[int, int]:
    """Add two fractions p1/q1 + p2/q2, return simplified."""
    num: int = p1 * q2 + p2 * q1
    den: int = q1 * q2
    g: int = num
    h: int = den
    if g < 0:
        g = -g
    if h < 0:
        h = -h
    while h != 0:
        temp: int = h
        h = g % h
        g = temp
    if g == 0:
        g = 1
    return (num // g, den // g)


def fraction_multiply(p1: int, q1: int, p2: int, q2: int) -> Tuple[int, int]:
    """Multiply two fractions."""
    num: int = p1 * p2
    den: int = q1 * q2
    g: int = num
    h: int = den
    if g < 0:
        g = -g
    if h < 0:
        h = -h
    while h != 0:
        temp: int = h
        h = g % h
        g = temp
    if g == 0:
        g = 1
    return (num // g, den // g)


def stern_brocot_path(p: int, q: int) -> str:
    """Find path in Stern-Brocot tree to fraction p/q."""
    if p <= 0 or q <= 0:
        return ""
    result: List[str] = []
    left_p: int = 0
    left_q: int = 1
    right_p: int = 1
    right_q: int = 0
    steps: int = 0
    while steps < 1000:
        med_p: int = left_p + right_p
        med_q: int = left_q + right_q
        if med_p * q == p * med_q:
            return "".join(result)
        elif p * med_q < med_p * q:
            result.append("L")
            right_p = med_p
            right_q = med_q
        else:
            result.append("R")
            left_p = med_p
            left_q = med_q
        steps += 1
    return "".join(result)


def egyptian_fraction(p: int, q: int) -> List[int]:
    """Express p/q as sum of unit fractions (greedy algorithm)."""
    result: List[int] = []
    num: int = p
    den: int = q
    steps: int = 0
    while num > 0 and steps < 100:
        ceil_d: int = (den + num - 1) // num
        result.append(ceil_d)
        num = num * ceil_d - den
        den = den * ceil_d
        g: int = num
        h: int = den
        if g < 0:
            g = -g
        while h != 0:
            temp: int = h
            h = g % h
            g = temp
        if g > 1:
            num = num // g
            den = den // g
        steps += 1
    return result


def test_fractions() -> bool:
    """Test continued fraction and rational arithmetic."""
    ok: bool = True
    pq: Tuple[int, int] = continued_fraction_sqrt2(5)
    if pq[0] <= 0:
        ok = False
    if pq[1] <= 0:
        ok = False
    added: Tuple[int, int] = fraction_add(1, 2, 1, 3)
    if added[0] != 5:
        ok = False
    if added[1] != 6:
        ok = False
    path: str = stern_brocot_path(3, 5)
    if len(path) == 0:
        ok = False
    return ok
