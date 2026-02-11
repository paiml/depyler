"""Coordinate transformations: translate, rotate 90 degrees, reflect.

Tests: translate, rotate_90_cw, rotate_90_ccw, reflect_x, reflect_y.
"""


def translate(points: list[list[int]], dx: int, dy: int) -> list[list[int]]:
    """Translate all points by (dx, dy)."""
    result: list[list[int]] = []
    i: int = 0
    while i < len(points):
        result.append([points[i][0] + dx, points[i][1] + dy])
        i = i + 1
    return result


def rotate_90_cw(points: list[list[int]]) -> list[list[int]]:
    """Rotate all points 90 degrees clockwise around origin: (x,y) -> (y,-x)."""
    result: list[list[int]] = []
    i: int = 0
    while i < len(points):
        x: int = points[i][0]
        y: int = points[i][1]
        result.append([y, -x])
        i = i + 1
    return result


def rotate_90_ccw(points: list[list[int]]) -> list[list[int]]:
    """Rotate all points 90 degrees counter-clockwise: (x,y) -> (-y,x)."""
    result: list[list[int]] = []
    i: int = 0
    while i < len(points):
        x: int = points[i][0]
        y: int = points[i][1]
        result.append([-y, x])
        i = i + 1
    return result


def reflect_x(points: list[list[int]]) -> list[list[int]]:
    """Reflect all points across x-axis: (x,y) -> (x,-y)."""
    result: list[list[int]] = []
    i: int = 0
    while i < len(points):
        result.append([points[i][0], -points[i][1]])
        i = i + 1
    return result


def reflect_y(points: list[list[int]]) -> list[list[int]]:
    """Reflect all points across y-axis: (x,y) -> (-x,y)."""
    result: list[list[int]] = []
    i: int = 0
    while i < len(points):
        result.append([-points[i][0], points[i][1]])
        i = i + 1
    return result


def manhattan_distance(p1: list[int], p2: list[int]) -> int:
    """Calculate Manhattan distance between two points."""
    dx: int = p1[0] - p2[0]
    dy: int = p1[1] - p2[1]
    if dx < 0:
        dx = -dx
    if dy < 0:
        dy = -dy
    return dx + dy


def test_module() -> int:
    """Test coordinate transformations."""
    ok: int = 0

    pts: list[list[int]] = [[1, 2], [3, 4]]

    t: list[list[int]] = translate(pts, 5, -1)
    if t == [[6, 1], [8, 3]]:
        ok = ok + 1

    r: list[list[int]] = rotate_90_cw([[1, 0]])
    if r == [[0, -1]]:
        ok = ok + 1

    r2: list[list[int]] = rotate_90_ccw([[1, 0]])
    if r2 == [[0, 1]]:
        ok = ok + 1

    rx: list[list[int]] = reflect_x([[3, 4]])
    if rx == [[3, -4]]:
        ok = ok + 1

    ry: list[list[int]] = reflect_y([[3, 4]])
    if ry == [[-3, 4]]:
        ok = ok + 1

    if manhattan_distance([0, 0], [3, 4]) == 7:
        ok = ok + 1

    return ok
