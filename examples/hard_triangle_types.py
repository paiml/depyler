"""Triangle classification and properties.

Tests: triangle type, perimeter, area by Heron's formula (integer approx).
"""


def classify_triangle(a: int, b: int, c: int) -> int:
    """Classify triangle: 0=invalid, 1=equilateral, 2=isosceles, 3=scalene."""
    if a + b <= c:
        return 0
    if a + c <= b:
        return 0
    if b + c <= a:
        return 0
    if a == b:
        if b == c:
            return 1
        return 2
    if a == c:
        return 2
    if b == c:
        return 2
    return 3


def triangle_perimeter(a: int, b: int, c: int) -> int:
    """Compute perimeter of a triangle."""
    return a + b + c


def is_right_triangle(a: int, b: int, c: int) -> int:
    """Check if triangle is right-angled. Returns 1 if yes."""
    sides: list[int] = [a, b, c]
    i: int = 0
    while i < 2:
        j: int = i + 1
        while j < 3:
            if sides[i] > sides[j]:
                tmp: int = sides[i]
                sides[i] = sides[j]
                sides[j] = tmp
            j = j + 1
        i = i + 1
    if sides[0] * sides[0] + sides[1] * sides[1] == sides[2] * sides[2]:
        return 1
    return 0


def heron_area_approx(a: int, b: int, c: int) -> int:
    """Approximate area using Heron's formula with integer sqrt.
    Returns 4 * area squared (to avoid floating point)."""
    s2: int = a + b + c
    val: int = s2 * (s2 - 2 * a) * (s2 - 2 * b) * (s2 - 2 * c)
    return val


def test_module() -> int:
    """Test triangle operations."""
    ok: int = 0
    if classify_triangle(3, 3, 3) == 1:
        ok = ok + 1
    if classify_triangle(3, 3, 5) == 2:
        ok = ok + 1
    if classify_triangle(3, 4, 5) == 3:
        ok = ok + 1
    if classify_triangle(1, 2, 3) == 0:
        ok = ok + 1
    if triangle_perimeter(3, 4, 5) == 12:
        ok = ok + 1
    if is_right_triangle(3, 4, 5) == 1:
        ok = ok + 1
    if is_right_triangle(3, 3, 3) == 0:
        ok = ok + 1
    return ok
