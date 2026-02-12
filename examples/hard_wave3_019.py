"""Numerical methods: Complex number arithmetic.

Tests: tuple-based complex representation, multiplication, division,
magnitude, phase approximation, Mandelbrot iteration.
"""

from typing import List, Tuple


def complex_add(ar: float, ai: float, br: float, bi: float) -> Tuple[float, float]:
    """Add two complex numbers (a + bi) + (c + di)."""
    return (ar + br, ai + bi)


def complex_sub(ar: float, ai: float, br: float, bi: float) -> Tuple[float, float]:
    """Subtract two complex numbers."""
    return (ar - br, ai - bi)


def complex_mul(ar: float, ai: float, br: float, bi: float) -> Tuple[float, float]:
    """Multiply two complex numbers (a+bi)(c+di)."""
    real: float = ar * br - ai * bi
    imag: float = ar * bi + ai * br
    return (real, imag)


def complex_div(ar: float, ai: float, br: float, bi: float) -> Tuple[float, float]:
    """Divide complex numbers (a+bi)/(c+di)."""
    denom: float = br * br + bi * bi
    if denom == 0.0:
        return (0.0, 0.0)
    real: float = (ar * br + ai * bi) / denom
    imag: float = (ai * br - ar * bi) / denom
    return (real, imag)


def complex_magnitude_sq(r: float, i: float) -> float:
    """Squared magnitude of complex number."""
    return r * r + i * i


def complex_magnitude(r: float, i: float) -> float:
    """Magnitude of complex number."""
    sq: float = r * r + i * i
    if sq == 0.0:
        return 0.0
    guess: float = sq / 2.0
    iterations: int = 0
    while iterations < 100:
        new_guess: float = (guess + sq / guess) / 2.0
        diff: float = new_guess - guess
        if diff < 0.0:
            diff = -diff
        if diff < 0.000001:
            return new_guess
        guess = new_guess
        iterations += 1
    return guess


def complex_conjugate(r: float, i: float) -> Tuple[float, float]:
    """Complex conjugate."""
    return (r, -i)


def complex_power(r: float, i: float, n: int) -> Tuple[float, float]:
    """Raise complex number to integer power n."""
    result_r: float = 1.0
    result_i: float = 0.0
    j: int = 0
    while j < n:
        new_r: float = result_r * r - result_i * i
        new_i: float = result_r * i + result_i * r
        result_r = new_r
        result_i = new_i
        j += 1
    return (result_r, result_i)


def mandelbrot_iterations(cr: float, ci: float, max_iter: int) -> int:
    """Count Mandelbrot iterations for point c = cr + ci*i."""
    zr: float = 0.0
    zi: float = 0.0
    n: int = 0
    while n < max_iter:
        if zr * zr + zi * zi > 4.0:
            return n
        new_zr: float = zr * zr - zi * zi + cr
        new_zi: float = 2.0 * zr * zi + ci
        zr = new_zr
        zi = new_zi
        n += 1
    return max_iter


def julia_iterations(zr: float, zi: float, cr: float, ci: float,
                     max_iter: int) -> int:
    """Count Julia set iterations."""
    r: float = zr
    i: float = zi
    n: int = 0
    while n < max_iter:
        if r * r + i * i > 4.0:
            return n
        new_r: float = r * r - i * i + cr
        new_i: float = 2.0 * r * i + ci
        r = new_r
        i = new_i
        n += 1
    return max_iter


def mandelbrot_row(ci: float, cr_start: float, cr_step: float,
                   width: int, max_iter: int) -> List[int]:
    """Compute one row of Mandelbrot set."""
    result: List[int] = []
    col: int = 0
    while col < width:
        cr: float = cr_start + float(col) * cr_step
        iters: int = mandelbrot_iterations(cr, ci, max_iter)
        result.append(iters)
        col += 1
    return result


def test_complex() -> bool:
    """Test complex number operations."""
    ok: bool = True
    prod: Tuple[float, float] = complex_mul(1.0, 2.0, 3.0, 4.0)
    diff: float = prod[0] - (-5.0)
    if diff < 0.0:
        diff = -diff
    if diff > 0.001:
        ok = False
    mag: float = complex_magnitude(3.0, 4.0)
    diff2: float = mag - 5.0
    if diff2 < 0.0:
        diff2 = -diff2
    if diff2 > 0.01:
        ok = False
    m_iter: int = mandelbrot_iterations(0.0, 0.0, 100)
    if m_iter != 100:
        ok = False
    m_iter2: int = mandelbrot_iterations(10.0, 10.0, 100)
    if m_iter2 >= 100:
        ok = False
    return ok
