"""Numerical methods: Geometric computations.

Tests: distance calculations, area computations, intersection detection,
coordinate transforms, convex hull primitives.
"""

from typing import List, Tuple


def distance_2d(x1: float, y1: float, x2: float, y2: float) -> float:
    """Euclidean distance between two 2D points."""
    dx: float = x2 - x1
    dy: float = y2 - y1
    sq: float = dx * dx + dy * dy
    if sq == 0.0:
        return 0.0
    guess: float = sq / 2.0
    iterations: int = 0
    while iterations < 100:
        new_guess: float = (guess + sq / guess) / 2.0
        diff: float = new_guess - guess
        if diff < 0.0:
            diff = -diff
        if diff < 0.000001:
            return new_guess
        guess = new_guess
        iterations += 1
    return guess


def manhattan_distance(x1: float, y1: float, x2: float, y2: float) -> float:
    """Manhattan distance between two 2D points."""
    dx: float = x2 - x1
    dy: float = y2 - y1
    if dx < 0.0:
        dx = -dx
    if dy < 0.0:
        dy = -dy
    return dx + dy


def triangle_area(x1: float, y1: float, x2: float, y2: float,
                  x3: float, y3: float) -> float:
    """Area of triangle using cross product formula."""
    area: float = (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)) / 2.0
    if area < 0.0:
        area = -area
    return area


def point_in_triangle(px: float, py: float,
                      x1: float, y1: float,
                      x2: float, y2: float,
                      x3: float, y3: float) -> bool:
    """Check if point (px,py) is inside triangle using area method."""
    total: float = triangle_area(x1, y1, x2, y2, x3, y3)
    a1: float = triangle_area(px, py, x2, y2, x3, y3)
    a2: float = triangle_area(x1, y1, px, py, x3, y3)
    a3: float = triangle_area(x1, y1, x2, y2, px, py)
    diff: float = (a1 + a2 + a3) - total
    if diff < 0.0:
        diff = -diff
    return diff < 0.0001


def line_intersection(x1: float, y1: float, x2: float, y2: float,
                      x3: float, y3: float, x4: float, y4: float) -> Tuple[float, float]:
    """Find intersection point of two lines. Returns (inf, inf) if parallel."""
    denom: float = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)
    if denom == 0.0:
        return (999999.0, 999999.0)
    t: float = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom
    ix: float = x1 + t * (x2 - x1)
    iy: float = y1 + t * (y2 - y1)
    return (ix, iy)


def polygon_area(xs: List[float], ys: List[float]) -> float:
    """Area of polygon using shoelace formula."""
    n: int = len(xs)
    if n < 3:
        return 0.0
    area: float = 0.0
    i: int = 0
    while i < n:
        j: int = (i + 1) % n
        area = area + xs[i] * ys[j]
        area = area - xs[j] * ys[i]
        i += 1
    area = area / 2.0
    if area < 0.0:
        area = -area
    return area


def ccw(x1: float, y1: float, x2: float, y2: float,
        x3: float, y3: float) -> int:
    """Counter-clockwise test. Returns 1 (CCW), -1 (CW), 0 (collinear)."""
    val: float = (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1)
    if val > 0.0001:
        return 1
    if val < -0.0001:
        return -1
    return 0


def perimeter(xs: List[float], ys: List[float]) -> float:
    """Compute perimeter of polygon."""
    n: int = len(xs)
    if n < 2:
        return 0.0
    total: float = 0.0
    i: int = 0
    while i < n:
        j: int = (i + 1) % n
        total = total + distance_2d(xs[i], ys[i], xs[j], ys[j])
        i += 1
    return total


def test_geometry() -> bool:
    """Test geometric computations."""
    ok: bool = True
    d: float = distance_2d(0.0, 0.0, 3.0, 4.0)
    diff: float = d - 5.0
    if diff < 0.0:
        diff = -diff
    if diff > 0.01:
        ok = False
    area: float = triangle_area(0.0, 0.0, 4.0, 0.0, 0.0, 3.0)
    diff2: float = area - 6.0
    if diff2 < 0.0:
        diff2 = -diff2
    if diff2 > 0.01:
        ok = False
    inside: bool = point_in_triangle(1.0, 1.0, 0.0, 0.0, 4.0, 0.0, 0.0, 4.0)
    if not inside:
        ok = False
    return ok
