"""Line intersection detection using integer arithmetic."""


def cross_product(ax: int, ay: int, bx: int, by: int) -> int:
    """Compute 2D cross product of vectors (ax,ay) and (bx,by)."""
    return ax * by - ay * bx


def sign_of(val: int) -> int:
    """Return sign: -1, 0, or 1."""
    if val > 0:
        return 1
    if val < 0:
        return -1
    return 0


def on_segment(px: int, py: int, qx: int, qy: int, rx: int, ry: int) -> int:
    """Check if point (rx,ry) lies on segment (px,py)-(qx,qy). Returns 1/0."""
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
    """Check if segment AB intersects segment CD. Returns 1/0."""
    d1x: int = bx - ax
    d1y: int = by - ay
    d2x: int = dx - cx
    d2y: int = dy - cy
    cp1: int = cross_product(d1x, d1y, cx - ax, cy - ay)
    cp2: int = cross_product(d1x, d1y, dx - ax, dy - ay)
    cp3: int = cross_product(d2x, d2y, ax - cx, ay - cy)
    cp4: int = cross_product(d2x, d2y, bx - cx, by - cy)
    s1: int = sign_of(cp1)
    s2: int = sign_of(cp2)
    s3: int = sign_of(cp3)
    s4: int = sign_of(cp4)
    if s1 != s2 and s3 != s4:
        return 1
    if s1 == 0 and on_segment(ax, ay, bx, by, cx, cy) == 1:
        return 1
    if s2 == 0 and on_segment(ax, ay, bx, by, dx, dy) == 1:
        return 1
    if s3 == 0 and on_segment(cx, cy, dx, dy, ax, ay) == 1:
        return 1
    if s4 == 0 and on_segment(cx, cy, dx, dy, bx, by) == 1:
        return 1
    return 0


def test_module() -> int:
    """Test line intersection operations."""
    passed: int = 0

    if cross_product(1, 0, 0, 1) == 1:
        passed = passed + 1

    if cross_product(1, 0, 1, 0) == 0:
        passed = passed + 1

    if segments_intersect(0, 0, 4, 4, 0, 4, 4, 0) == 1:
        passed = passed + 1

    if segments_intersect(0, 0, 1, 1, 2, 2, 3, 3) == 0:
        passed = passed + 1

    if segments_intersect(0, 0, 2, 0, 1, 0, 3, 0) == 1:
        passed = passed + 1

    if sign_of(5) == 1:
        passed = passed + 1

    if sign_of(-3) == -1:
        passed = passed + 1

    if on_segment(0, 0, 4, 4, 2, 2) == 1:
        passed = passed + 1

    return passed
