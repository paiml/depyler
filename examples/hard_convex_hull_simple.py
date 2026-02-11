"""Convex hull area and perimeter using simplified coordinate geometry.

Tests: polygon area, perimeter, point containment.
"""


def polygon_area_2x(xs: list[int], ys: list[int]) -> int:
    """Compute 2x the area of polygon using shoelace formula.
    Returns absolute value."""
    n: int = len(xs)
    area: int = 0
    i: int = 0
    while i < n:
        j: int = (i + 1) % n
        area = area + xs[i] * ys[j]
        area = area - xs[j] * ys[i]
        i = i + 1
    if area < 0:
        area = -area
    return area


def polygon_perimeter_sq(xs: list[int], ys: list[int]) -> int:
    """Compute sum of squared edge lengths of polygon."""
    n: int = len(xs)
    total: int = 0
    i: int = 0
    while i < n:
        j: int = (i + 1) % n
        dx: int = xs[j] - xs[i]
        dy: int = ys[j] - ys[i]
        total = total + dx * dx + dy * dy
        i = i + 1
    return total


def bounding_box_area(xs: list[int], ys: list[int]) -> int:
    """Compute area of bounding box."""
    min_x: int = xs[0]
    max_x: int = xs[0]
    min_y: int = ys[0]
    max_y: int = ys[0]
    i: int = 1
    while i < len(xs):
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


def test_module() -> int:
    """Test convex hull operations."""
    ok: int = 0
    xs: list[int] = [0, 4, 4, 0]
    ys: list[int] = [0, 0, 3, 3]
    if polygon_area_2x(xs, ys) == 24:
        ok = ok + 1
    if bounding_box_area(xs, ys) == 12:
        ok = ok + 1
    psq: int = polygon_perimeter_sq(xs, ys)
    if psq == 50:
        ok = ok + 1
    txs: list[int] = [0, 3, 0]
    tys: list[int] = [0, 0, 4]
    if polygon_area_2x(txs, tys) == 12:
        ok = ok + 1
    return ok
