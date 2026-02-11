# Type inference test: Parameter type inference from calls
# Strategy: Parameters inferred from how functions are called


def increment(x):
    """Type of x inferred from: called with int literals."""
    return x + 1


def decrement(x):
    """Type of x inferred from: called with int literals."""
    return x - 1


def negate(x):
    """Type of x inferred from arithmetic."""
    return 0 - x


def triple(x):
    """Type of x inferred from multiplication."""
    return x * 3


def safe_divide(a, b):
    """Types inferred from int arithmetic and comparisons."""
    if b == 0:
        return 0
    return a // b


def bounded_add(a, b, limit):
    """All types inferred from int operations."""
    result = a + b
    if result > limit:
        return limit
    if result < 0 - limit:
        return 0 - limit
    return result


def accumulate_steps(start, step, count):
    """All types inferred from loop and arithmetic."""
    result = start
    i = 0
    while i < count:
        result = result + step
        i = i + 1
    return result


def repeat_double(x, times):
    """Repeatedly double x."""
    result = x
    i = 0
    while i < times:
        result = result * 2
        i = i + 1
    return result


def sum_chain(a, b, c, d, e):
    """Five params, all inferred from addition."""
    return a + b + c + d + e


def cascade_max(a, b, c):
    """Find max of three values, all inferred."""
    best = a
    if b > best:
        best = b
    if c > best:
        best = c
    return best


def test_module() -> int:
    """Test parameter type inference from call sites."""
    total: int = 0

    # increment / decrement tests
    if increment(5) == 6:
        total = total + 1
    if decrement(5) == 4:
        total = total + 1

    # negate tests
    if negate(5) == 0 - 5:
        total = total + 1
    if negate(0 - 3) == 3:
        total = total + 1

    # triple tests
    if triple(7) == 21:
        total = total + 1

    # safe_divide tests
    if safe_divide(10, 3) == 3:
        total = total + 1
    if safe_divide(10, 0) == 0:
        total = total + 1

    # bounded_add tests
    if bounded_add(50, 30, 100) == 80:
        total = total + 1
    if bounded_add(80, 50, 100) == 100:
        total = total + 1

    # accumulate_steps tests
    if accumulate_steps(0, 5, 10) == 50:
        total = total + 1
    if accumulate_steps(100, 0 - 10, 5) == 50:
        total = total + 1

    # repeat_double tests
    if repeat_double(1, 10) == 1024:
        total = total + 1
    if repeat_double(3, 0) == 3:
        total = total + 1

    # sum_chain test
    if sum_chain(1, 2, 3, 4, 5) == 15:
        total = total + 1

    # cascade_max tests
    if cascade_max(3, 7, 5) == 7:
        total = total + 1
    if cascade_max(10, 2, 8) == 10:
        total = total + 1

    return total
