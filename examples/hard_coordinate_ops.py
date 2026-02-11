"""2D coordinate calculations: distance, midpoint, and area operations."""


def manhattan_distance(x1: int, y1: int, x2: int, y2: int) -> int:
    """Calculate Manhattan distance between two points."""
    dx: int = x2 - x1
    if dx < 0:
        dx = -dx
    dy: int = y2 - y1
    if dy < 0:
        dy = -dy
    return dx + dy


def chebyshev_distance(x1: int, y1: int, x2: int, y2: int) -> int:
    """Calculate Chebyshev distance between two points."""
    dx: int = x2 - x1
    if dx < 0:
        dx = -dx
    dy: int = y2 - y1
    if dy < 0:
        dy = -dy
    if dx > dy:
        return dx
    return dy


def triangle_area_doubled(x1: int, y1: int, x2: int, y2: int, x3: int, y3: int) -> int:
    """Calculate twice the area of a triangle from coordinates (avoids floats)."""
    area: int = x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)
    if area < 0:
        area = -area
    return area


def is_collinear(x1: int, y1: int, x2: int, y2: int, x3: int, y3: int) -> int:
    """Check if three points are collinear. Returns 1 if collinear, 0 otherwise."""
    area: int = triangle_area_doubled(x1, y1, x2, y2, x3, y3)
    if area == 0:
        return 1
    return 0


def test_module() -> int:
    """Test coordinate operations."""
    ok: int = 0

    if manhattan_distance(0, 0, 3, 4) == 7:
        ok = ok + 1

    if manhattan_distance(-1, -1, 2, 3) == 7:
        ok = ok + 1

    if chebyshev_distance(0, 0, 3, 4) == 4:
        ok = ok + 1

    if chebyshev_distance(0, 0, 5, 2) == 5:
        ok = ok + 1

    if triangle_area_doubled(0, 0, 4, 0, 0, 3) == 12:
        ok = ok + 1

    if is_collinear(0, 0, 1, 1, 2, 2) == 1:
        ok = ok + 1

    if is_collinear(0, 0, 1, 1, 2, 3) == 0:
        ok = ok + 1

    return ok
