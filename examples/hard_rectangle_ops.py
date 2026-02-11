"""Rectangle overlap and area computations."""


def rect_area(x1: int, y1: int, x2: int, y2: int) -> int:
    """Compute area of rectangle defined by corners (x1,y1) and (x2,y2)."""
    width: int = x2 - x1
    height: int = y2 - y1
    if width < 0:
        width = -width
    if height < 0:
        height = -height
    return width * height


def rects_overlap(ax1: int, ay1: int, ax2: int, ay2: int, bx1: int, by1: int, bx2: int, by2: int) -> int:
    """Check if two rectangles overlap. Returns 1/0."""
    if ax1 >= bx2 or bx1 >= ax2:
        return 0
    if ay1 >= by2 or by1 >= ay2:
        return 0
    return 1


def overlap_area(ax1: int, ay1: int, ax2: int, ay2: int, bx1: int, by1: int, bx2: int, by2: int) -> int:
    """Compute area of overlap between two rectangles."""
    if rects_overlap(ax1, ay1, ax2, ay2, bx1, by1, bx2, by2) == 0:
        return 0
    ox1: int = ax1
    if bx1 > ox1:
        ox1 = bx1
    oy1: int = ay1
    if by1 > oy1:
        oy1 = by1
    ox2: int = ax2
    if bx2 < ox2:
        ox2 = bx2
    oy2: int = ay2
    if by2 < oy2:
        oy2 = by2
    return rect_area(ox1, oy1, ox2, oy2)


def perimeter(x1: int, y1: int, x2: int, y2: int) -> int:
    """Compute perimeter of rectangle."""
    width: int = x2 - x1
    height: int = y2 - y1
    if width < 0:
        width = -width
    if height < 0:
        height = -height
    return 2 * (width + height)


def test_module() -> int:
    """Test rectangle operations."""
    passed: int = 0

    if rect_area(0, 0, 3, 4) == 12:
        passed = passed + 1

    if rect_area(1, 1, 1, 1) == 0:
        passed = passed + 1

    if rects_overlap(0, 0, 2, 2, 1, 1, 3, 3) == 1:
        passed = passed + 1

    if rects_overlap(0, 0, 1, 1, 2, 2, 3, 3) == 0:
        passed = passed + 1

    if overlap_area(0, 0, 2, 2, 1, 1, 3, 3) == 1:
        passed = passed + 1

    if overlap_area(0, 0, 1, 1, 2, 2, 3, 3) == 0:
        passed = passed + 1

    if perimeter(0, 0, 3, 4) == 14:
        passed = passed + 1

    if overlap_area(0, 0, 4, 4, 1, 1, 3, 3) == 4:
        passed = passed + 1

    return passed
