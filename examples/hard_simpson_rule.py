# Numerical integration (Simpson's rule) using integer arithmetic
# All functions work with plain integers; results scaled where noted


def eval_poly2(a: int, b: int, c: int, x: int) -> int:
    # Evaluate a*x*x + b*x + c with x in units of 1
    return a * x * x + b * x + c


def simpson_integrate_scaled(a: int, b: int, c: int, lo: int, hi: int, n: int, scale: int) -> int:
    # Simpson's 1/3 rule for f(x) = a*x^2 + b*x + c over [lo, hi]
    # x-values are in units of scale (lo=0, hi=scale means [0, 1])
    # Returns integral * scale^2 / scale = integral * scale
    if n % 2 != 0:
        n = n + 1
    h: int = (hi - lo) // n
    f_lo: int = a * lo * lo + b * lo * scale + c * scale * scale
    f_hi: int = a * hi * hi + b * hi * scale + c * scale * scale
    total: int = f_lo + f_hi
    i: int = 1
    while i < n:
        x: int = lo + i * h
        fx: int = a * x * x + b * x * scale + c * scale * scale
        if i % 2 == 0:
            total = total + 2 * fx
        else:
            total = total + 4 * fx
        i = i + 1
    return total * h // (3 * scale * scale)


def trapezoid_integrate_scaled(a: int, b: int, c: int, lo: int, hi: int, n: int, scale: int) -> int:
    # Trapezoidal rule for f(x) = a*x^2 + b*x + c
    h: int = (hi - lo) // n
    f_lo: int = a * lo * lo + b * lo * scale + c * scale * scale
    f_hi: int = a * hi * hi + b * hi * scale + c * scale * scale
    total: int = (f_lo + f_hi) // 2
    i: int = 1
    while i < n:
        x: int = lo + i * h
        fx: int = a * x * x + b * x * scale + c * scale * scale
        total = total + fx
        i = i + 1
    return total * h // (scale * scale)


def abs_val(x: int) -> int:
    if x < 0:
        return -x
    return x


def test_module() -> int:
    passed: int = 0
    scale: int = 1000

    # Test 1: integral of 1 from 0 to 1 = 1 => result = scale
    r: int = simpson_integrate_scaled(0, 0, 1, 0, scale, 10, scale)
    if abs_val(r - scale) < 10:
        passed = passed + 1

    # Test 2: integral of x from 0 to 1 = 0.5 => result = scale/2 = 500
    r = simpson_integrate_scaled(0, 1, 0, 0, scale, 10, scale)
    if abs_val(r - 500) < 10:
        passed = passed + 1

    # Test 3: integral of x^2 from 0 to 1 = 1/3 => result ~ 333
    r = simpson_integrate_scaled(1, 0, 0, 0, scale, 100, scale)
    if abs_val(r - 333) < 10:
        passed = passed + 1

    # Test 4: integral of x^2 from 0 to 2 = 8/3 ~ 2666
    r = simpson_integrate_scaled(1, 0, 0, 0, 2 * scale, 100, scale)
    if abs_val(r - 2666) < 20:
        passed = passed + 1

    # Test 5: trapezoid of constant = exact
    r = trapezoid_integrate_scaled(0, 0, 1, 0, scale, 10, scale)
    if abs_val(r - scale) < 10:
        passed = passed + 1

    # Test 6: trapezoid of x from 0 to 1 = 0.5
    r = trapezoid_integrate_scaled(0, 1, 0, 0, scale, 100, scale)
    if abs_val(r - 500) < 10:
        passed = passed + 1

    # Test 7: integral of 2x+3 from 0 to 1 = x^2 + 3x |_0^1 = 4 => 4*scale
    r = simpson_integrate_scaled(0, 2, 3, 0, scale, 10, scale)
    if abs_val(r - 4 * scale) < 20:
        passed = passed + 1

    return passed
