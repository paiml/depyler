"""Vector math class with add, dot product, magnitude.

Tests: class arithmetic, multiple method returns, squared magnitude.
"""


class Vec2:
    """2D vector with typed operations."""

    def __init__(self, x: float, y: float) -> None:
        self.x: float = x
        self.y: float = y

    def get_x(self) -> float:
        return self.x

    def get_y(self) -> float:
        return self.y

    def magnitude_squared(self) -> float:
        return self.x * self.x + self.y * self.y

    def negate(self) -> None:
        self.x = -self.x
        self.y = -self.y


def vec_dot(ax: float, ay: float, bx: float, by: float) -> float:
    """Compute dot product of two vectors."""
    return ax * bx + ay * by


def vec_cross_z(ax: float, ay: float, bx: float, by: float) -> float:
    """Compute z-component of cross product."""
    return ax * by - ay * bx


def vec_add_x(ax: float, bx: float) -> float:
    """Add x components."""
    return ax + bx


def vec_add_y(ay: float, by: float) -> float:
    """Add y components."""
    return ay + by


def vec_scale(val: float, factor: float) -> float:
    """Scale a component."""
    return val * factor


def test_module() -> int:
    """Test vector operations."""
    ok: int = 0

    v = Vec2(3.0, 4.0)
    if v.get_x() == 3.0:
        ok += 1
    if v.get_y() == 4.0:
        ok += 1
    if v.magnitude_squared() == 25.0:
        ok += 1

    d: float = vec_dot(1.0, 0.0, 0.0, 1.0)
    if d == 0.0:
        ok += 1

    d2: float = vec_dot(3.0, 4.0, 3.0, 4.0)
    if d2 == 25.0:
        ok += 1

    cz: float = vec_cross_z(1.0, 0.0, 0.0, 1.0)
    if cz == 1.0:
        ok += 1

    sx: float = vec_add_x(3.0, 7.0)
    if sx == 10.0:
        ok += 1

    return ok
