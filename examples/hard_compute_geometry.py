"""Computational geometry: distance, area, convex hull check, cross product.

Tests: distance_sq, triangle_area_x2, is_convex, cross_product, polygon_area_x2.
"""


def distance_squared(x1: int, y1: int, x2: int, y2: int) -> int:
    """Squared Euclidean distance (avoids float)."""
    dx: int = x2 - x1
    dy: int = y2 - y1
    return dx * dx + dy * dy


def cross_product(ox: int, oy: int, ax: int, ay: int, bx: int, by: int) -> int:
    """Cross product of vectors OA and OB. Positive = CCW."""
    return (ax - ox) * (by - oy) - (ay - oy) * (bx - ox)


def triangle_area_x2(x1: int, y1: int, x2: int, y2: int, x3: int, y3: int) -> int:
    """Twice the signed area of triangle. Absolute value = twice area."""
    area: int = x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)
    if area < 0:
        area = 0 - area
    return area


def polygon_area_x2(xs: list[int], ys: list[int]) -> int:
    """Twice the area of a simple polygon using shoelace formula."""
    n: int = len(xs)
    if n < 3:
        return 0
    area: int = 0
    i: int = 0
    while i < n:
        j: int = (i + 1) % n
        area = area + xs[i] * ys[j]
        area = area - xs[j] * ys[i]
        i = i + 1
    if area < 0:
        area = 0 - area
    return area


def is_convex_polygon(xs: list[int], ys: list[int]) -> int:
    """Check if polygon (given as coordinate lists) is convex. Returns 1 or 0."""
    n: int = len(xs)
    if n < 3:
        return 0
    sign: int = 0
    i: int = 0
    while i < n:
        i1: int = (i + 1) % n
        i2: int = (i + 2) % n
        cp: int = cross_product(xs[i], ys[i], xs[i1], ys[i1], xs[i2], ys[i2])
        if cp != 0:
            if sign == 0:
                if cp > 0:
                    sign = 1
                else:
                    sign = -1
            else:
                if sign == 1:
                    if cp < 0:
                        return 0
                else:
                    if cp > 0:
                        return 0
        i = i + 1
    return 1


def point_in_triangle(px: int, py: int, x1: int, y1: int, x2: int, y2: int, x3: int, y3: int) -> int:
    """Check if point (px,py) is inside triangle. Returns 1 or 0."""
    d1: int = cross_product(x1, y1, x2, y2, px, py)
    d2: int = cross_product(x2, y2, x3, y3, px, py)
    d3: int = cross_product(x3, y3, x1, y1, px, py)
    has_neg: int = 0
    has_pos: int = 0
    if d1 < 0:
        has_neg = 1
    if d1 > 0:
        has_pos = 1
    if d2 < 0:
        has_neg = 1
    if d2 > 0:
        has_pos = 1
    if d3 < 0:
        has_neg = 1
    if d3 > 0:
        has_pos = 1
    if has_neg == 1:
        if has_pos == 1:
            return 0
    return 1


def collinear(x1: int, y1: int, x2: int, y2: int, x3: int, y3: int) -> int:
    """Check if three points are collinear. Returns 1 or 0."""
    cp: int = cross_product(x1, y1, x2, y2, x3, y3)
    if cp == 0:
        return 1
    return 0


def perimeter_squared_sum(xs: list[int], ys: list[int]) -> int:
    """Sum of squared edge lengths of polygon."""
    n: int = len(xs)
    total: int = 0
    i: int = 0
    while i < n:
        j: int = (i + 1) % n
        total = total + distance_squared(xs[i], ys[i], xs[j], ys[j])
        i = i + 1
    return total


def test_module() -> int:
    """Test geometry algorithms."""
    passed: int = 0

    if distance_squared(0, 0, 3, 4) == 25:
        passed = passed + 1

    a2: int = triangle_area_x2(0, 0, 4, 0, 0, 3)
    if a2 == 12:
        passed = passed + 1

    xs: list[int] = [0, 4, 4, 0]
    ys: list[int] = [0, 0, 3, 3]
    pa: int = polygon_area_x2(xs, ys)
    if pa == 24:
        passed = passed + 1

    if is_convex_polygon(xs, ys) == 1:
        passed = passed + 1

    pit: int = point_in_triangle(1, 1, 0, 0, 4, 0, 0, 4)
    if pit == 1:
        passed = passed + 1

    if collinear(0, 0, 1, 1, 2, 2) == 1:
        passed = passed + 1

    if collinear(0, 0, 1, 0, 0, 1) == 0:
        passed = passed + 1

    return passed
