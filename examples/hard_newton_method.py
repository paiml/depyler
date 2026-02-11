# Newton's method for sqrt and cube root (using integer-scaled arithmetic)
# We use fixed-point arithmetic with scale factor 10000


def newton_sqrt(n: int, scale: int) -> int:
    # Compute sqrt(n) * scale using Newton's method
    if n == 0:
        return 0
    # x_{k+1} = (x_k + n*scale^2/x_k) / 2
    x: int = n * scale
    iterations: int = 0
    while iterations < 50:
        x_new: int = (x + n * scale * scale // x) // 2
        diff: int = x_new - x
        if diff < 0:
            diff = -diff
        if diff <= 1:
            x = x_new
            break
        x = x_new
        iterations = iterations + 1
    return x


def newton_cbrt(n: int, scale: int) -> int:
    # Compute cbrt(n) * scale using Newton's method
    if n == 0:
        return 0
    # x_{k+1} = (2*x_k + n*scale^3/x_k^2) / 3
    x: int = n * scale
    iterations: int = 0
    while iterations < 80:
        x_sq: int = x * x
        if x_sq == 0:
            break
        x_new: int = (2 * x + n * scale * scale * scale // x_sq) // 3
        diff: int = x_new - x
        if diff < 0:
            diff = -diff
        if diff <= 1:
            x = x_new
            break
        x = x_new
        iterations = iterations + 1
    return x


def isqrt(n: int) -> int:
    # Integer square root
    if n < 0:
        return -1
    if n == 0:
        return 0
    x: int = n
    y: int = (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x


def abs_val(x: int) -> int:
    if x < 0:
        return -x
    return x


def test_module() -> int:
    passed: int = 0
    scale: int = 1000

    # Test 1: sqrt(4) ~= 2.000
    r: int = newton_sqrt(4, scale)
    if abs_val(r - 2000) <= 2:
        passed = passed + 1

    # Test 2: sqrt(9) ~= 3.000
    r = newton_sqrt(9, scale)
    if abs_val(r - 3000) <= 2:
        passed = passed + 1

    # Test 3: sqrt(2) ~= 1.414 => 1414
    r = newton_sqrt(2, scale)
    if abs_val(r - 1414) <= 2:
        passed = passed + 1

    # Test 4: sqrt(0) = 0
    if newton_sqrt(0, scale) == 0:
        passed = passed + 1

    # Test 5: isqrt(100) = 10
    if isqrt(100) == 10:
        passed = passed + 1

    # Test 6: isqrt(26) = 5
    if isqrt(26) == 5:
        passed = passed + 1

    # Test 7: cbrt(8) ~= 2.000
    c: int = newton_cbrt(8, scale)
    if abs_val(c - 2000) <= 5:
        passed = passed + 1

    return passed
