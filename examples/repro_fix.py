# DEPYLER-0719: Bare tuple annotation maps to () instead of actual type
# Python pattern: result: tuple = func_returning_tuple()
# Problem: tuple → () but func returns (T1, T2, T3)
# Expected: Infer tuple type from function return or usage

def get_point() -> tuple:
    """Return a 2D point as tuple."""
    x: float = 1.0
    y: float = 2.0
    return (x, y)

def use_point() -> float:
    """Use the point tuple."""
    point: tuple = get_point()
    return point[0] + point[1]
