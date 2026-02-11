"""Point class with geometric methods.

Tests: class with multiple methods, arithmetic on fields, float computation.
"""


class Point:
    """2D point with typed fields."""

    def __init__(self, x: float, y: float) -> None:
        self.x: float = x
        self.y: float = y

    def translate(self, dx: float, dy: float) -> None:
        self.x = self.x + dx
        self.y = self.y + dy

    def scale(self, factor: float) -> None:
        self.x = self.x * factor
        self.y = self.y * factor

    def get_x(self) -> float:
        return self.x

    def get_y(self) -> float:
        return self.y


def distance_squared(x1: float, y1: float, x2: float, y2: float) -> float:
    """Compute squared distance between two points."""
    dx: float = x2 - x1
    dy: float = y2 - y1
    return dx * dx + dy * dy


def midpoint_x(x1: float, x2: float) -> float:
    """Compute midpoint x-coordinate."""
    return (x1 + x2) / 2.0


def midpoint_y(y1: float, y2: float) -> float:
    """Compute midpoint y-coordinate."""
    return (y1 + y2) / 2.0


def manhattan_distance(x1: float, y1: float, x2: float, y2: float) -> float:
    """Compute Manhattan distance between two points."""
    dx: float = x2 - x1
    dy: float = y2 - y1
    if dx < 0.0:
        dx = -dx
    if dy < 0.0:
        dy = -dy
    return dx + dy


def test_module() -> int:
    """Test point operations."""
    ok: int = 0

    p = Point(3.0, 4.0)
    if p.get_x() == 3.0:
        ok += 1
    if p.get_y() == 4.0:
        ok += 1

    p.translate(1.0, 2.0)
    if p.get_x() == 4.0:
        ok += 1

    d: float = distance_squared(0.0, 0.0, 3.0, 4.0)
    if d == 25.0:
        ok += 1

    mx: float = midpoint_x(0.0, 10.0)
    if mx == 5.0:
        ok += 1

    md: float = manhattan_distance(0.0, 0.0, 3.0, 4.0)
    if md == 7.0:
        ok += 1

    return ok
