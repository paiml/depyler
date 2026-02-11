"""Rectangle class with area, perimeter, and point containment.

Tests: class methods, boolean returns, comparison logic.
"""


class Rectangle:
    """Axis-aligned rectangle."""

    def __init__(self, x: float, y: float, w: float, h: float) -> None:
        self.x: float = x
        self.y: float = y
        self.w: float = w
        self.h: float = h

    def area(self) -> float:
        return self.w * self.h

    def perimeter(self) -> float:
        return 2.0 * (self.w + self.h)

    def get_right(self) -> float:
        return self.x + self.w

    def get_top(self) -> float:
        return self.y + self.h


def contains_point(rx: float, ry: float, rw: float, rh: float,
                   px: float, py: float) -> bool:
    """Check if a point is inside a rectangle."""
    if px < rx:
        return False
    if px > rx + rw:
        return False
    if py < ry:
        return False
    if py > ry + rh:
        return False
    return True


def overlap_area(x1: float, y1: float, w1: float, h1: float,
                 x2: float, y2: float, w2: float, h2: float) -> float:
    """Compute overlap area of two rectangles."""
    left: float = x1
    if x2 > left:
        left = x2
    right1: float = x1 + w1
    right2: float = x2 + w2
    right: float = right1
    if right2 < right:
        right = right2
    bottom: float = y1
    if y2 > bottom:
        bottom = y2
    top1: float = y1 + h1
    top2: float = y2 + h2
    top: float = top1
    if top2 < top:
        top = top2
    ow: float = right - left
    oh: float = top - bottom
    if ow <= 0.0:
        return 0.0
    if oh <= 0.0:
        return 0.0
    return ow * oh


def test_module() -> int:
    """Test rectangle operations."""
    ok: int = 0

    r = Rectangle(0.0, 0.0, 10.0, 5.0)
    if r.area() == 50.0:
        ok += 1
    if r.perimeter() == 30.0:
        ok += 1
    if r.get_right() == 10.0:
        ok += 1
    if r.get_top() == 5.0:
        ok += 1

    if contains_point(0.0, 0.0, 10.0, 5.0, 5.0, 2.0):
        ok += 1
    if not contains_point(0.0, 0.0, 10.0, 5.0, 15.0, 2.0):
        ok += 1

    oa: float = overlap_area(0.0, 0.0, 10.0, 10.0, 5.0, 5.0, 10.0, 10.0)
    if oa == 25.0:
        ok += 1

    return ok
