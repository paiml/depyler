"""Harmonic series and overtone computations using integer arithmetic.

Tests: harmonics, overtone series, harmonic mean, Fourier coefficients.
Scale factor 1000 for fixed-point.
"""


def nth_harmonic(fundamental: int, n: int) -> int:
    """Compute nth harmonic frequency = n * fundamental."""
    return n * fundamental


def overtone_number(harmonic_num: int) -> int:
    """Overtone number = harmonic number - 1."""
    if harmonic_num <= 0:
        return 0
    return harmonic_num - 1


def harmonic_series_sum(fundamental: int, num_terms: int) -> int:
    """Sum of harmonic series frequencies: f1 + 2*f1 + ... + n*f1."""
    total: int = 0
    i: int = 1
    while i <= num_terms:
        total = total + i * fundamental
        i = i + 1
    return total


def harmonic_mean_two(a: int, b: int) -> int:
    """Harmonic mean of two values = 2*a*b/(a+b). Fixed-point scale 1000."""
    denom: int = a + b
    if denom == 0:
        return 0
    result: int = (2 * a * b) // denom
    return result


def harmonic_mean_list(values: list[int]) -> int:
    """Harmonic mean of a list: n / sum(1/xi). Fixed-point scale 1000."""
    n: int = len(values)
    if n == 0:
        return 0
    reciprocal_sum: int = 0
    i: int = 0
    while i < n:
        val: int = values[i]
        if val == 0:
            return 0
        reciprocal_sum = reciprocal_sum + (1000 * 1000) // val
        i = i + 1
    if reciprocal_sum == 0:
        return 0
    result: int = (n * 1000 * 1000) // reciprocal_sum
    return result


def fourier_square_coefficient(harmonic_num: int, amplitude: int) -> int:
    """Fourier coefficient for square wave: 4A/(n*pi) for odd n, 0 for even.
    pi approx 3142/1000. Fixed-point scale 1000."""
    if harmonic_num <= 0:
        return 0
    if harmonic_num % 2 == 0:
        return 0
    result: int = (4 * amplitude * 1000) // (harmonic_num * 3142)
    return result


def fourier_sawtooth_coefficient(harmonic_num: int, amplitude: int) -> int:
    """Fourier coefficient for sawtooth wave: 2A/(n*pi) * (-1)^(n+1).
    Fixed-point scale 1000."""
    if harmonic_num <= 0:
        return 0
    magnitude: int = (2 * amplitude * 1000) // (harmonic_num * 3142)
    if harmonic_num % 2 == 0:
        return 0 - magnitude
    return magnitude


def total_harmonic_distortion(harmonics: list[int]) -> int:
    """THD = sqrt(sum of squares of harmonics) / fundamental * 1000.
    harmonics[0] is fundamental, rest are higher harmonics. Scale 1000."""
    if len(harmonics) < 2:
        return 0
    fund: int = harmonics[0]
    if fund == 0:
        return 0
    sum_sq: int = 0
    i: int = 1
    while i < len(harmonics):
        val: int = harmonics[i]
        sum_sq = sum_sq + val * val
        i = i + 1
    guess: int = sum_sq
    if guess == 0:
        return 0
    iterations: int = 0
    target: int = sum_sq
    while iterations < 50:
        if guess == 0:
            return 0
        next_g: int = (guess + target // guess) // 2
        diff: int = next_g - guess
        if diff < 0:
            diff = 0 - diff
        if diff < 2:
            return (next_g * 1000) // fund
        guess = next_g
        iterations = iterations + 1
    return (guess * 1000) // fund


def pipe_harmonic(length: int, harmonic_num: int, open_pipe: int) -> int:
    """Frequency of harmonic in a pipe. open_pipe=1 for open, 0 for closed.
    Open pipe: all harmonics. Closed pipe: odd harmonics only.
    f = n * v / (2*L) for open, f = n * v / (4*L) for closed.
    v = 343000 (speed of sound in mm/s scale 1000). Fixed-point."""
    speed: int = 343000
    if length == 0:
        return 0
    if open_pipe == 1:
        result: int = (harmonic_num * speed) // (2 * length)
        return result
    else:
        if harmonic_num % 2 == 0:
            return 0
        result2: int = (harmonic_num * speed) // (4 * length)
        return result2


def test_module() -> int:
    """Test harmonic computations."""
    ok: int = 0
    h3: int = nth_harmonic(440, 3)
    if h3 == 1320:
        ok = ok + 1
    ov: int = overtone_number(3)
    if ov == 2:
        ok = ok + 1
    hs: int = harmonic_series_sum(100, 4)
    if hs == 1000:
        ok = ok + 1
    hm: int = harmonic_mean_two(1000, 1000)
    if hm == 1000:
        ok = ok + 1
    fc1: int = fourier_square_coefficient(1, 1000)
    if fc1 > 1200 and fc1 < 1300:
        ok = ok + 1
    fc2: int = fourier_square_coefficient(2, 1000)
    if fc2 == 0:
        ok = ok + 1
    pipe_open: int = pipe_harmonic(1000, 1, 1)
    if pipe_open > 0:
        ok = ok + 1
    return ok
