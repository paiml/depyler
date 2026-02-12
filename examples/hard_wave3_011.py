"""Numerical methods: Statistical computations.

Tests: running statistics, variance computation, histogram building,
percentile calculation, correlation coefficients.
"""

from typing import List, Tuple


def mean(values: List[float]) -> float:
    """Compute arithmetic mean."""
    if len(values) == 0:
        return 0.0
    total: float = 0.0
    for v in values:
        total = total + v
    return total / float(len(values))


def variance(values: List[float]) -> float:
    """Compute population variance using two-pass algorithm."""
    n: int = len(values)
    if n == 0:
        return 0.0
    m: float = mean(values)
    total: float = 0.0
    for v in values:
        diff: float = v - m
        total = total + diff * diff
    return total / float(n)


def standard_deviation(values: List[float]) -> float:
    """Compute standard deviation via Newton's method sqrt."""
    var: float = variance(values)
    if var <= 0.0:
        return 0.0
    guess: float = var / 2.0
    iterations: int = 0
    while iterations < 100:
        new_guess: float = (guess + var / guess) / 2.0
        diff: float = new_guess - guess
        if diff < 0.0:
            diff = -diff
        if diff < 0.000001:
            return new_guess
        guess = new_guess
        iterations += 1
    return guess


def welford_mean_var(values: List[float]) -> Tuple[float, float]:
    """Welford's online algorithm for mean and variance."""
    n: int = 0
    m: float = 0.0
    m2: float = 0.0
    for x in values:
        n += 1
        delta: float = x - m
        m = m + delta / float(n)
        delta2: float = x - m
        m2 = m2 + delta * delta2
    if n < 2:
        return (m, 0.0)
    return (m, m2 / float(n))


def median_sorted(values: List[float]) -> float:
    """Compute median of a sorted list."""
    n: int = len(values)
    if n == 0:
        return 0.0
    if n % 2 == 1:
        return values[n // 2]
    return (values[n // 2 - 1] + values[n // 2]) / 2.0


def histogram_counts(values: List[float], num_bins: int,
                     lo: float, hi: float) -> List[int]:
    """Build histogram with fixed bin count."""
    bins: List[int] = []
    i: int = 0
    while i < num_bins:
        bins.append(0)
        i += 1
    rng: float = hi - lo
    if rng <= 0.0:
        return bins
    for v in values:
        if v >= lo and v < hi:
            idx: int = int((v - lo) * float(num_bins) / rng)
            if idx >= num_bins:
                idx = num_bins - 1
            bins[idx] = bins[idx] + 1
    return bins


def correlation_coefficient(xs: List[float], ys: List[float]) -> float:
    """Compute Pearson correlation coefficient."""
    n: int = len(xs)
    if n == 0 or n != len(ys):
        return 0.0
    mx: float = mean(xs)
    my: float = mean(ys)
    sum_xy: float = 0.0
    sum_x2: float = 0.0
    sum_y2: float = 0.0
    i: int = 0
    while i < n:
        dx: float = xs[i] - mx
        dy: float = ys[i] - my
        sum_xy = sum_xy + dx * dy
        sum_x2 = sum_x2 + dx * dx
        sum_y2 = sum_y2 + dy * dy
        i += 1
    denom_sq: float = sum_x2 * sum_y2
    if denom_sq <= 0.0:
        return 0.0
    denom: float = denom_sq
    guess: float = denom / 2.0
    iterations: int = 0
    while iterations < 100:
        new_guess: float = (guess + denom / guess) / 2.0
        diff: float = new_guess - guess
        if diff < 0.0:
            diff = -diff
        if diff < 0.000001:
            return sum_xy / new_guess
        guess = new_guess
        iterations += 1
    return sum_xy / guess


def test_statistics() -> bool:
    """Test statistical computations."""
    ok: bool = True
    m: float = mean([1.0, 2.0, 3.0, 4.0, 5.0])
    diff: float = m - 3.0
    if diff < 0.0:
        diff = -diff
    if diff > 0.001:
        ok = False
    v: float = variance([2.0, 4.0, 6.0])
    if v < 0.0:
        ok = False
    med: float = median_sorted([1.0, 2.0, 3.0])
    diff2: float = med - 2.0
    if diff2 < 0.0:
        diff2 = -diff2
    if diff2 > 0.001:
        ok = False
    return ok
