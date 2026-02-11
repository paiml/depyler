def cross_product_2d(ox: int, oy: int, ax: int, ay: int, bx: int, by: int) -> int:
    return (ax - ox) * (by - oy) - (ay - oy) * (bx - ox)


def on_segment(px: int, py: int, qx: int, qy: int, rx: int, ry: int) -> int:
    min_x: int = px
    if qx < min_x:
        min_x = qx
    max_x: int = px
    if qx > max_x:
        max_x = qx
    min_y: int = py
    if qy < min_y:
        min_y = qy
    max_y: int = py
    if qy > max_y:
        max_y = qy
    if rx >= min_x and rx <= max_x and ry >= min_y and ry <= max_y:
        return 1
    return 0


def segments_intersect(ax: int, ay: int, bx: int, by: int, cx: int, cy: int, dx: int, dy: int) -> int:
    d1: int = cross_product_2d(cx, cy, dx, dy, ax, ay)
    d2: int = cross_product_2d(cx, cy, dx, dy, bx, by)
    d3: int = cross_product_2d(ax, ay, bx, by, cx, cy)
    d4: int = cross_product_2d(ax, ay, bx, by, dx, dy)
    if ((d1 > 0 and d2 < 0) or (d1 < 0 and d2 > 0)) and ((d3 > 0 and d4 < 0) or (d3 < 0 and d4 > 0)):
        return 1
    if d1 == 0 and on_segment(cx, cy, dx, dy, ax, ay) == 1:
        return 1
    if d2 == 0 and on_segment(cx, cy, dx, dy, bx, by) == 1:
        return 1
    if d3 == 0 and on_segment(ax, ay, bx, by, cx, cy) == 1:
        return 1
    if d4 == 0 and on_segment(ax, ay, bx, by, dx, dy) == 1:
        return 1
    return 0


def collinear(ax: int, ay: int, bx: int, by: int, cx: int, cy: int) -> int:
    cp: int = cross_product_2d(ax, ay, bx, by, cx, cy)
    if cp == 0:
        return 1
    return 0


def distance_squared(ax: int, ay: int, bx: int, by: int) -> int:
    dx: int = bx - ax
    dy: int = by - ay
    return dx * dx + dy * dy


def test_module() -> int:
    passed: int = 0
    if segments_intersect(0, 0, 10, 10, 0, 10, 10, 0) == 1:
        passed = passed + 1
    if segments_intersect(0, 0, 1, 1, 2, 2, 3, 3) == 0:
        passed = passed + 1
    if collinear(0, 0, 1, 1, 2, 2) == 1:
        passed = passed + 1
    if collinear(0, 0, 1, 1, 1, 0) == 0:
        passed = passed + 1
    if cross_product_2d(0, 0, 1, 0, 0, 1) == 1:
        passed = passed + 1
    if distance_squared(0, 0, 3, 4) == 25:
        passed = passed + 1
    if segments_intersect(0, 0, 2, 2, 1, 1, 3, 3) == 1:
        passed = passed + 1
    return passed
