# Linear and Lagrange interpolation using integer-scaled arithmetic


def linear_interp(x0: int, y0: int, x1: int, y1: int, x: int, scale: int) -> int:
    # Linear interpolation: y = y0 + (y1-y0)*(x-x0)/(x1-x0)
    # All values in scaled units
    if x1 == x0:
        return y0
    return y0 + (y1 - y0) * (x - x0) // (x1 - x0)


def lagrange_interp(xs: list[int], ys: list[int], x: int, scale: int) -> int:
    # Lagrange interpolation at point x
    # All values in scaled units
    n: int = len(xs)
    result: int = 0
    i: int = 0
    while i < n:
        numerator: int = scale
        denominator: int = scale
        j: int = 0
        while j < n:
            if j != i:
                numerator = numerator * (x - xs[j]) // scale
                denominator = denominator * (xs[i] - xs[j]) // scale
            j = j + 1
        if denominator != 0:
            result = result + ys[i] * numerator // denominator
        i = i + 1
    return result


def piecewise_linear(xs: list[int], ys: list[int], x: int, scale: int) -> int:
    # Piecewise linear interpolation
    n: int = len(xs)
    if n == 0:
        return 0
    if x <= xs[0]:
        return ys[0]
    if x >= xs[n - 1]:
        return ys[n - 1]
    i: int = 0
    while i < n - 1:
        if x >= xs[i] and x <= xs[i + 1]:
            return linear_interp(xs[i], ys[i], xs[i + 1], ys[i + 1], x, scale)
        i = i + 1
    return ys[n - 1]


def abs_val(x: int) -> int:
    if x < 0:
        return -x
    return x


def test_module() -> int:
    passed: int = 0
    scale: int = 1000

    # Test 1: linear interp midpoint
    r: int = linear_interp(0, 0, 2000, 2000, 1000, scale)
    if abs_val(r - 1000) < 5:
        passed = passed + 1

    # Test 2: linear interp at endpoints
    r = linear_interp(0, 100, 1000, 500, 0, scale)
    if r == 100:
        passed = passed + 1

    # Test 3: linear interp at other endpoint
    r = linear_interp(0, 100, 1000, 500, 1000, scale)
    if r == 500:
        passed = passed + 1

    # Test 4: Lagrange through 2 points = linear
    xs: list[int] = [0, 2000]
    ys: list[int] = [0, 4000]
    r = lagrange_interp(xs, ys, 1000, scale)
    if abs_val(r - 2000) < 50:
        passed = passed + 1

    # Test 5: Lagrange through 3 points (quadratic)
    # Points: (0,0), (1,1), (2,4) => y = x^2
    xs3: list[int] = [0, 1000, 2000]
    ys3: list[int] = [0, 1000, 4000]
    # At x=1.5 => 2.25 => 2250
    r = lagrange_interp(xs3, ys3, 1500, scale)
    if abs_val(r - 2250) < 100:
        passed = passed + 1

    # Test 6: piecewise linear
    pxs: list[int] = [0, 1000, 2000, 3000]
    pys: list[int] = [0, 1000, 1000, 2000]
    r = piecewise_linear(pxs, pys, 500, scale)
    if abs_val(r - 500) < 50:
        passed = passed + 1

    # Test 7: piecewise linear flat segment
    r = piecewise_linear(pxs, pys, 1500, scale)
    if abs_val(r - 1000) < 50:
        passed = passed + 1

    return passed
