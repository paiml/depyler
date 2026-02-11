"""Area computations: polygons, circles (integer approx), and composite shapes."""


def rectangle_area(width: int, height: int) -> int:
    """Calculate area of a rectangle."""
    if width < 0 or height < 0:
        return 0
    return width * height


def triangle_area_heron_x4(a: int, b: int, c: int) -> int:
    """Calculate 4 * area using Heron's formula with integer arithmetic.
    Uses 16*area^2 = 2a^2*b^2 + 2b^2*c^2 + 2c^2*a^2 - a^4 - b^4 - c^4
    Returns sqrt approximation * 4 of this."""
    a2: int = a * a
    b2: int = b * b
    c2: int = c * c
    val: int = 2 * a2 * b2 + 2 * b2 * c2 + 2 * c2 * a2 - a2 * a2 - b2 * b2 - c2 * c2
    if val <= 0:
        return 0
    # Integer square root
    root: int = 0
    while (root + 1) * (root + 1) <= val:
        root = root + 1
    return root


def shoelace_area_x2(xs: list[int], ys: list[int]) -> int:
    """Calculate twice the area of a polygon using the shoelace formula.
    Vertices should be in order (clockwise or counter-clockwise)."""
    n: int = len(xs)
    if n < 3:
        return 0
    area: int = 0
    i: int = 0
    while i < n:
        next_i: int = (i + 1) % n
        area = area + xs[i] * ys[next_i]
        area = area - xs[next_i] * ys[i]
        i = i + 1
    if area < 0:
        area = -area
    return area


def circle_area_approx(radius: int) -> int:
    """Approximate circle area using integer math: pi ~= 355/113.
    Returns area * 113 / 355 simplification -> radius^2 * 355 / 113."""
    return radius * radius * 355 // 113


def test_module() -> int:
    """Test area calculation functions."""
    ok: int = 0

    if rectangle_area(5, 3) == 15:
        ok = ok + 1

    if rectangle_area(-1, 5) == 0:
        ok = ok + 1

    # Triangle 3-4-5: area = 6, 4*area = 24, 16*area^2 = 576
    t_area: int = triangle_area_heron_x4(3, 4, 5)
    if t_area == 24:
        ok = ok + 1

    # Square with vertices (0,0), (2,0), (2,2), (0,2)
    xs: list[int] = [0, 2, 2, 0]
    ys: list[int] = [0, 0, 2, 2]
    if shoelace_area_x2(xs, ys) == 8:
        ok = ok + 1

    # Circle with radius 10: area ~= 314
    c_area: int = circle_area_approx(10)
    if c_area > 310 and c_area < 320:
        ok = ok + 1

    if circle_area_approx(0) == 0:
        ok = ok + 1

    return ok
