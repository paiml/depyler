"""Coordinate geometry patterns.

Tests: polygon area (shoelace), point-in-polygon, convex hull perimeter,
Manhattan distance, and line intersection detection.
"""


def polygon_area_2x(xs: list[int], ys: list[int]) -> int:
    """Compute 2x the area of a polygon using Shoelace formula (returns absolute value)."""
    n: int = len(xs)
    if n < 3:
        return 0
    area: int = 0
    i: int = 0
    while i < n:
        j: int = (i + 1) % n
        area = area + xs[i] * ys[j] - xs[j] * ys[i]
        i = i + 1
    if area < 0:
        area = 0 - area
    return area


def triangle_area_2x(x1: int, y1: int, x2: int, y2: int, x3: int, y3: int) -> int:
    """Compute 2x the area of a triangle from coordinates."""
    area: int = x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)
    if area < 0:
        area = 0 - area
    return area


def manhattan_distance(x1: int, y1: int, x2: int, y2: int) -> int:
    """Compute Manhattan distance between two points."""
    dx: int = x2 - x1
    if dx < 0:
        dx = 0 - dx
    dy: int = y2 - y1
    if dy < 0:
        dy = 0 - dy
    return dx + dy


def perimeter_of_polygon(xs: list[int], ys: list[int]) -> int:
    """Compute approximate perimeter using Manhattan distance between vertices."""
    n: int = len(xs)
    if n < 2:
        return 0
    total: int = 0
    i: int = 0
    while i < n:
        j: int = (i + 1) % n
        total = total + manhattan_distance(xs[i], ys[i], xs[j], ys[j])
        i = i + 1
    return total


def cross_product_2d(ox: int, oy: int, ax: int, ay: int, bx: int, by: int) -> int:
    """Cross product of vectors OA and OB."""
    return (ax - ox) * (by - oy) - (ay - oy) * (bx - ox)


def collinear(x1: int, y1: int, x2: int, y2: int, x3: int, y3: int) -> bool:
    """Check if three points are collinear."""
    return cross_product_2d(x1, y1, x2, y2, x3, y3) == 0


def bounding_box_area(xs: list[int], ys: list[int]) -> int:
    """Compute area of axis-aligned bounding box of a set of points."""
    n: int = len(xs)
    if n == 0:
        return 0
    min_x: int = xs[0]
    max_x: int = xs[0]
    min_y: int = ys[0]
    max_y: int = ys[0]
    i: int = 1
    while i < n:
        if xs[i] < min_x:
            min_x = xs[i]
        if xs[i] > max_x:
            max_x = xs[i]
        if ys[i] < min_y:
            min_y = ys[i]
        if ys[i] > max_y:
            max_y = ys[i]
        i = i + 1
    return (max_x - min_x) * (max_y - min_y)


def test_module() -> bool:
    """Test all coordinate geometry functions."""
    ok: bool = True

    area: int = polygon_area_2x([0, 4, 4, 0], [0, 0, 3, 3])
    if area != 24:
        ok = False

    ta: int = triangle_area_2x(0, 0, 4, 0, 0, 3)
    if ta != 12:
        ok = False

    if manhattan_distance(1, 2, 4, 6) != 7:
        ok = False

    peri: int = perimeter_of_polygon([0, 4, 4, 0], [0, 0, 3, 3])
    if peri != 14:
        ok = False

    if not collinear(0, 0, 1, 1, 2, 2):
        ok = False
    if collinear(0, 0, 1, 1, 2, 3):
        ok = False

    bb: int = bounding_box_area([1, 5, 3], [2, 8, 4])
    if bb != 24:
        ok = False

    return ok
