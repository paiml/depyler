"""Probability computations using integer arithmetic (fractions as numerator/denominator)."""


def gcd(a: int, b: int) -> int:
    """Greatest common divisor for reducing fractions."""
    if a < 0:
        a = -a
    if b < 0:
        b = -b
    while b > 0:
        temp: int = b
        b = a % b
        a = temp
    return a


def combinations(n: int, r: int) -> int:
    """Calculate C(n, r) = n! / (r! * (n-r)!)."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    if r > n - r:
        r = n - r
    result: int = 1
    i: int = 0
    while i < r:
        result = result * (n - i)
        result = result // (i + 1)
        i = i + 1
    return result


def binomial_probability_num(n: int, trial_k: int, success_num: int, success_den: int) -> int:
    """Numerator of binomial probability P(X=k) = C(n,k) * p^k * (1-p)^(n-k).
    Probability given as fraction success_num/success_den.
    Returns numerator when denominator is success_den^n."""
    c: int = combinations(n, trial_k)
    # p^k
    pk: int = 1
    i: int = 0
    while i < trial_k:
        pk = pk * success_num
        i = i + 1
    # (1-p)^(n-k) = ((den-num)/den)^(n-k)
    fail_num: int = success_den - success_num
    fail_power: int = 1
    j: int = 0
    diff: int = n - trial_k
    while j < diff:
        fail_power = fail_power * fail_num
        j = j + 1
    return c * pk * fail_power


def expected_value_x100(values: list[int], weights: list[int]) -> int:
    """Calculate expected value * 100 given values and weights (integer weights)."""
    total_weight: int = 0
    weighted_sum: int = 0
    i: int = 0
    while i < len(values):
        weighted_sum = weighted_sum + values[i] * weights[i]
        total_weight = total_weight + weights[i]
        i = i + 1
    if total_weight == 0:
        return 0
    return weighted_sum * 100 // total_weight


def test_module() -> int:
    """Test probability calculation functions."""
    ok: int = 0

    if gcd(12, 8) == 4:
        ok = ok + 1

    if combinations(5, 2) == 10:
        ok = ok + 1

    if combinations(10, 3) == 120:
        ok = ok + 1

    # P(X=2) for n=3, p=1/2: C(3,2)*1^2*1^1 = 3
    # denominator = 2^3 = 8, so P = 3/8
    num: int = binomial_probability_num(3, 2, 1, 2)
    if num == 3:
        ok = ok + 1

    vals: list[int] = [1, 2, 3]
    wts: list[int] = [1, 1, 1]
    if expected_value_x100(vals, wts) == 200:
        ok = ok + 1

    if combinations(0, 0) == 1:
        ok = ok + 1

    return ok
