# Type inference test: Cross-function type propagation
# Strategy: typed function returns feed into untyped function params


def square(x: int) -> int:
    """Fully typed: return x squared."""
    return x * x


def cube(x: int) -> int:
    """Fully typed: return x cubed."""
    return x * x * x


def add_values(a, b):
    """Untyped: add two values (types propagated from callers)."""
    return a + b


def subtract_values(a, b):
    """Untyped: subtract."""
    return a - b


def multiply_values(a, b):
    """Untyped: multiply."""
    return a * b


def max_of_two(a, b):
    """Untyped: return the larger value."""
    if a >= b:
        return a
    return b


def min_of_two(a, b):
    """Untyped: return the smaller value."""
    if a <= b:
        return a
    return b


def compose_add_square(x):
    """Untyped: compute square(x) + square(x+1)."""
    s1 = square(x)
    s2 = square(x + 1)
    return add_values(s1, s2)


def compose_diff_cubes(a, b):
    """Untyped: compute cube(a) - cube(b)."""
    c1 = cube(a)
    c2 = cube(b)
    return subtract_values(c1, c2)


def weighted_average(v1, w1, v2, w2):
    """Untyped: integer weighted average."""
    total_weight = add_values(w1, w2)
    if total_weight == 0:
        return 0
    numerator = add_values(multiply_values(v1, w1), multiply_values(v2, w2))
    return numerator // total_weight


def test_module() -> int:
    """Test cross-function type propagation."""
    total: int = 0

    # Basic typed functions
    if square(5) == 25:
        total = total + 1
    if cube(3) == 27:
        total = total + 1

    # Untyped with typed inputs
    if add_values(square(3), square(4)) == 25:
        total = total + 1
    if subtract_values(cube(3), cube(2)) == 19:
        total = total + 1

    # max/min of typed returns
    if max_of_two(square(3), square(2)) == 9:
        total = total + 1
    if min_of_two(square(3), square(2)) == 4:
        total = total + 1

    # compose functions
    if compose_add_square(3) == 25:
        total = total + 1
    if compose_diff_cubes(3, 2) == 19:
        total = total + 1

    # weighted_average
    if weighted_average(10, 3, 20, 1) == 12:
        total = total + 1

    # Direct untyped calls with literals
    if add_values(100, 200) == 300:
        total = total + 1
    if multiply_values(7, 8) == 56:
        total = total + 1
    if max_of_two(42, 17) == 42:
        total = total + 1

    return total
