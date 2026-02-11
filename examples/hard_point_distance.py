"""Point distance computations using integer arithmetic."""


def distance_squared(x1: int, y1: int, x2: int, y2: int) -> int:
    """Compute squared Euclidean distance between two points."""
    dx: int = x2 - x1
    dy: int = y2 - y1
    return dx * dx + dy * dy


def manhattan_distance(x1: int, y1: int, x2: int, y2: int) -> int:
    """Compute Manhattan distance between two points."""
    dx: int = x2 - x1
    dy: int = y2 - y1
    if dx < 0:
        dx = -dx
    if dy < 0:
        dy = -dy
    return dx + dy


def closest_pair_brute(xs: list[int], ys: list[int]) -> int:
    """Find squared distance of closest pair of points (brute force)."""
    n: int = len(xs)
    if n < 2:
        return 0
    min_dist: int = distance_squared(xs[0], ys[0], xs[1], ys[1])
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            d: int = distance_squared(xs[i], ys[i], xs[j], ys[j])
            if d < min_dist:
                min_dist = d
            j = j + 1
        i = i + 1
    return min_dist


def is_inside_circle(px: int, py: int, cx: int, cy: int, radius_sq: int) -> int:
    """Check if point (px,py) is inside circle centered at (cx,cy). Returns 1/0."""
    d: int = distance_squared(px, py, cx, cy)
    if d <= radius_sq:
        return 1
    return 0


def test_module() -> int:
    """Test point distance operations."""
    passed: int = 0

    if distance_squared(0, 0, 3, 4) == 25:
        passed = passed + 1

    if distance_squared(1, 1, 1, 1) == 0:
        passed = passed + 1

    if manhattan_distance(0, 0, 3, 4) == 7:
        passed = passed + 1

    if manhattan_distance(1, 2, 4, 6) == 7:
        passed = passed + 1

    xs: list[int] = [0, 3, 1]
    ys: list[int] = [0, 4, 1]
    cp: int = closest_pair_brute(xs, ys)
    if cp == 2:
        passed = passed + 1

    if is_inside_circle(1, 1, 0, 0, 4) == 1:
        passed = passed + 1

    if is_inside_circle(5, 5, 0, 0, 4) == 0:
        passed = passed + 1

    if manhattan_distance(0, 0, 0, 0) == 0:
        passed = passed + 1

    return passed
