"""Complex number operations using pairs of floats.

Tests: add, multiply, magnitude, conjugate.
"""


def complex_add_real(a_re: float, a_im: float, b_re: float, b_im: float) -> float:
    """Real part of complex addition."""
    return a_re + b_re


def complex_add_imag(a_re: float, a_im: float, b_re: float, b_im: float) -> float:
    """Imaginary part of complex addition."""
    return a_im + b_im


def complex_mul_real(a_re: float, a_im: float, b_re: float, b_im: float) -> float:
    """Real part of complex multiplication: ac - bd."""
    return a_re * b_re - a_im * b_im


def complex_mul_imag(a_re: float, a_im: float, b_re: float, b_im: float) -> float:
    """Imaginary part of complex multiplication: ad + bc."""
    return a_re * b_im + a_im * b_re


def complex_magnitude_sq(re: float, im: float) -> float:
    """Square of magnitude: re^2 + im^2."""
    return re * re + im * im


def complex_conjugate_imag(im: float) -> float:
    """Imaginary part of conjugate."""
    return -im


def complex_power_real(re: float, im: float, n: int) -> float:
    """Real part of (re + im*i)^n computed iteratively."""
    result_re: float = 1.0
    result_im: float = 0.0
    i: int = 0
    while i < n:
        new_re: float = result_re * re - result_im * im
        new_im: float = result_re * im + result_im * re
        result_re = new_re
        result_im = new_im
        i = i + 1
    return result_re


def test_module() -> None:
    assert complex_add_real(1.0, 2.0, 3.0, 4.0) == 4.0
    assert complex_add_imag(1.0, 2.0, 3.0, 4.0) == 6.0
    mr: float = complex_mul_real(1.0, 2.0, 3.0, 4.0)
    assert mr > -5.1 and mr < -4.9
    mi: float = complex_mul_imag(1.0, 2.0, 3.0, 4.0)
    assert mi > 9.9 and mi < 10.1
    mag: float = complex_magnitude_sq(3.0, 4.0)
    assert mag > 24.9 and mag < 25.1
    assert complex_conjugate_imag(5.0) == -5.0
    pr: float = complex_power_real(0.0, 1.0, 2)
    assert pr > -1.1 and pr < -0.9
