"""Newton's method for root finding using integer-scaled arithmetic."""


def newton_sqrt_scaled(n: int, scale: int) -> int:
    """Integer Newton's method for sqrt(n)*scale.
    Computes x where x/scale approximates sqrt(n)."""
    if n <= 0:
        return 0
    x: int = n * scale
    i: int = 0
    while i < 30:
        x_new: int = (x + n * scale * scale // x) // 2
        if x_new == x:
            return x
        x = x_new
        i = i + 1
    return x


def isqrt(n: int) -> int:
    """Integer square root: largest x such that x*x <= n."""
    if n < 0:
        return 0
    if n == 0:
        return 0
    x: int = n
    y: int = (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x


def cube_root_newton(n: int) -> int:
    """Integer cube root approximation via Newton's method."""
    if n <= 0:
        return 0
    x: int = n
    i: int = 0
    while i < 40:
        x_new: int = (2 * x + n // (x * x)) // 3
        if x_new >= x:
            return x
        x = x_new
        i = i + 1
    return x


def is_perfect_square(n: int) -> int:
    """Returns 1 if n is a perfect square."""
    if n < 0:
        return 0
    r: int = isqrt(n)
    if r * r == n:
        return 1
    return 0


def reciprocal_scaled(n: int, scale: int) -> int:
    """Compute scale*scale/n using Newton's method for 1/n."""
    if n == 0:
        return 0
    x: int = scale * scale // n
    return x


def test_module() -> int:
    """Test Newton's method functions."""
    ok: int = 0
    if isqrt(16) == 4:
        ok = ok + 1
    if isqrt(25) == 5:
        ok = ok + 1
    if isqrt(26) == 5:
        ok = ok + 1
    if is_perfect_square(144) == 1:
        ok = ok + 1
    if is_perfect_square(145) == 0:
        ok = ok + 1
    if cube_root_newton(27) == 3:
        ok = ok + 1
    return ok
