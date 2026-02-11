"""Computational geometry and math patterns for transpiler testing.

Tests: float/int coercion, numeric type inference, epsilon comparisons,
trigonometric approximations, coordinate geometry, vector math,
matrix operations, numerical methods, interpolation.
"""

from typing import Dict, List, Optional, Tuple


PI: float = 3.14159265358979323846
EPSILON: float = 1e-9


def float_abs(x: float) -> float:
    """Absolute value for float."""
    if x < 0.0:
        return -x
    return x


def float_eq(a: float, b: float) -> bool:
    """Check approximate float equality."""
    return float_abs(a - b) < EPSILON


def newton_sqrt(x: float) -> float:
    """Square root using Newton's method (pattern 10)."""
    if x <= 0.0:
        return 0.0
    guess: float = x / 2.0
    if guess == 0.0:
        guess = 1.0
    for _ in range(100):
        next_guess: float = (guess + x / guess) / 2.0
        if float_abs(next_guess - guess) < EPSILON:
            return next_guess
        guess = next_guess
    return guess


def sin_approx(x: float) -> float:
    """Approximate sine using Taylor series (trig helper)."""
    tau: float = 2.0 * PI
    while x > PI:
        x -= tau
    while x < -PI:
        x += tau
    x2: float = x * x
    x3: float = x2 * x
    x5: float = x3 * x2
    x7: float = x5 * x2
    x9: float = x7 * x2
    return x - x3 / 6.0 + x5 / 120.0 - x7 / 5040.0 + x9 / 362880.0


def cos_approx(x: float) -> float:
    """Approximate cosine using Taylor series (trig helper)."""
    tau: float = 2.0 * PI
    while x > PI:
        x -= tau
    while x < -PI:
        x += tau
    x2: float = x * x
    x4: float = x2 * x2
    x6: float = x4 * x2
    x8: float = x6 * x2
    return 1.0 - x2 / 2.0 + x4 / 24.0 - x6 / 720.0 + x8 / 40320.0


def acos_approx(x: float) -> float:
    """Approximate acos (Abramowitz & Stegun polynomial)."""
    if x < -1.0:
        x = -1.0
    if x > 1.0:
        x = 1.0
    negate: float = 0.0
    if x < 0.0:
        negate = PI
        x = -x
    ret: float = -0.0187293 * x
    ret = ret + 0.0742610
    ret = ret * x
    ret = ret - 0.2121144
    ret = ret * x
    ret = ret + 1.5707288
    sq: float = newton_sqrt(1.0 - x)
    ret = ret * sq
    if negate > 0.0:
        return negate + ret
    return ret


# --- 1. Point distance calculations (2D, 3D, Manhattan, Chebyshev) ---


def distance_2d(x1: float, y1: float, x2: float, y2: float) -> float:
    """Euclidean distance between two 2D points."""
    dx: float = x2 - x1
    dy: float = y2 - y1
    return newton_sqrt(dx * dx + dy * dy)


def distance_3d(
    x1: float, y1: float, z1: float, x2: float, y2: float, z2: float
) -> float:
    """Euclidean distance between two 3D points."""
    dx: float = x2 - x1
    dy: float = y2 - y1
    dz: float = z2 - z1
    return newton_sqrt(dx * dx + dy * dy + dz * dz)


def manhattan_distance(x1: float, y1: float, x2: float, y2: float) -> float:
    """Manhattan (L1) distance between two 2D points."""
    return float_abs(x2 - x1) + float_abs(y2 - y1)


def chebyshev_distance(x1: float, y1: float, x2: float, y2: float) -> float:
    """Chebyshev (L-infinity) distance between two 2D points."""
    adx: float = float_abs(x2 - x1)
    ady: float = float_abs(y2 - y1)
    if adx >= ady:
        return adx
    return ady


# --- 2. Line intersection detection ---


def line_intersection(
    x1: float, y1: float, x2: float, y2: float,
    x3: float, y3: float, x4: float, y4: float,
) -> Optional[Tuple[float, float]]:
    """Find intersection point of two line segments, or None."""
    denom: float = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)
    if float_abs(denom) < EPSILON:
        return None
    t_num: float = (x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)
    u_num: float = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3))
    t: float = t_num / denom
    u: float = u_num / denom
    if t < 0.0 or t > 1.0 or u < 0.0 or u > 1.0:
        return None
    ix: float = x1 + t * (x2 - x1)
    iy: float = y1 + t * (y2 - y1)
    return (ix, iy)


# --- 3. Polygon area (shoelace formula) ---


def polygon_area(xs: List[float], ys: List[float]) -> float:
    """Compute area of a polygon using the shoelace formula."""
    n: int = len(xs)
    if n < 3:
        return 0.0
    area: float = 0.0
    j: int = n - 1
    for i in range(n):
        area += (xs[j] + xs[i]) * (ys[j] - ys[i])
        j = i
    return float_abs(area) / 2.0


# --- 4. Convex hull (Jarvis march) ---


def convex_hull_area(xs: List[float], ys: List[float]) -> float:
    """Compute area of the convex hull via gift wrapping + shoelace."""
    n: int = len(xs)
    if n < 3:
        return 0.0
    start: int = 0
    for i in range(1, n):
        if xs[i] < xs[start]:
            start = i
        elif xs[i] == xs[start] and ys[i] < ys[start]:
            start = i
    hull_xs: List[float] = []
    hull_ys: List[float] = []
    current: int = start
    while True:
        hull_xs.append(xs[current])
        hull_ys.append(ys[current])
        candidate: int = 0
        for i in range(1, n):
            if candidate == current:
                candidate = i
                continue
            cross: float = (xs[candidate] - xs[current]) * (ys[i] - ys[current]) - (ys[candidate] - ys[current]) * (xs[i] - xs[current])
            if cross < 0.0:
                candidate = i
        current = candidate
        if current == start:
            break
    return polygon_area(hull_xs, hull_ys)


# --- 5. Point in polygon (ray casting) ---


def point_in_polygon(
    px: float, py: float, xs: List[float], ys: List[float]
) -> bool:
    """Test if point (px, py) is inside polygon using ray casting."""
    n: int = len(xs)
    inside: bool = False
    j: int = n - 1
    for i in range(n):
        if (ys[i] > py) != (ys[j] > py):
            slope: float = (xs[j] - xs[i]) * (py - ys[i]) / (ys[j] - ys[i]) + xs[i]
            if px < slope:
                inside = not inside
        j = i
    return inside


# --- 6. Triangle properties ---


def triangle_area_coords(
    x1: float, y1: float, x2: float, y2: float, x3: float, y3: float
) -> float:
    """Area of triangle from coordinates."""
    area: float = x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)
    return float_abs(area) / 2.0


def triangle_classify(a: float, b: float, c: float) -> str:
    """Classify triangle by sides and angles combined."""
    if a <= 0.0 or b <= 0.0 or c <= 0.0:
        return "invalid"
    if a + b <= c or b + c <= a or a + c <= b:
        return "invalid"
    if float_eq(a, b) and float_eq(b, c):
        return "equilateral"
    # Sort so s3 is largest
    s1: float = a
    s2: float = b
    s3: float = c
    if s1 > s2:
        tmp: float = s1
        s1 = s2
        s2 = tmp
    if s2 > s3:
        tmp2: float = s2
        s2 = s3
        s3 = tmp2
    if s1 > s2:
        tmp3: float = s1
        s1 = s2
        s2 = tmp3
    sq_sum: float = s1 * s1 + s2 * s2
    sq_big: float = s3 * s3
    if float_eq(sq_big, sq_sum):
        return "right"
    if sq_big > sq_sum:
        return "obtuse"
    return "acute"


def heron_area(a: float, b: float, c: float) -> float:
    """Triangle area using Heron's formula."""
    s: float = (a + b + c) / 2.0
    val: float = s * (s - a) * (s - b) * (s - c)
    if val < 0.0:
        return 0.0
    return newton_sqrt(val)


# --- 7. Circle operations ---


def circle_area(radius: float) -> float:
    """Area of a circle."""
    return PI * radius * radius


def circles_intersect(
    x1: float, y1: float, r1: float, x2: float, y2: float, r2: float
) -> bool:
    """Check if two circles intersect or touch."""
    d: float = distance_2d(x1, y1, x2, y2)
    return d <= r1 + r2 and d >= float_abs(r1 - r2)


# --- 8. Matrix determinant (2x2, 3x3) ---


def det_2x2(a: float, b: float, c: float, d: float) -> float:
    """Determinant of a 2x2 matrix [[a,b],[c,d]]."""
    return a * d - b * c


def det_3x3(
    a: float, b: float, c: float,
    d: float, e: float, f: float,
    g: float, h: float, k: float,
) -> float:
    """Determinant of a 3x3 matrix [[a,b,c],[d,e,f],[g,h,k]]."""
    return a * (e * k - f * h) - b * (d * k - f * g) + c * (d * h - e * g)


# --- 9. Vector operations ---


def dot_product(xs: List[float], ys: List[float]) -> float:
    """Dot product of two vectors."""
    result: float = 0.0
    n: int = len(xs)
    for i in range(n):
        result += xs[i] * ys[i]
    return result


def cross_product_3d(
    ax: float, ay: float, az: float, bx: float, by: float, bz: float
) -> Tuple[float, float, float]:
    """Cross product of two 3D vectors."""
    cx: float = ay * bz - az * by
    cy: float = az * bx - ax * bz
    cz: float = ax * by - ay * bx
    return (cx, cy, cz)


def vector_normalize(xs: List[float]) -> List[float]:
    """Normalize a vector to unit length."""
    sq_sum: float = 0.0
    for x in xs:
        sq_sum += x * x
    mag: float = newton_sqrt(sq_sum)
    if mag < EPSILON:
        return xs
    result: List[float] = []
    for x in xs:
        result.append(x / mag)
    return result


# --- 11. Polynomial evaluation (Horner's method) ---


def horner_eval(coeffs: List[float], x: float) -> float:
    """Evaluate polynomial using Horner's method.

    coeffs[0] is the highest degree coefficient.
    """
    result: float = 0.0
    for c in coeffs:
        result = result * x + c
    return result


# --- 12. Numerical integration ---


def trapezoidal_rule(
    coeffs: List[float], a: float, b: float, n: int
) -> float:
    """Approximate integral of polynomial using the trapezoidal rule."""
    if n <= 0:
        return 0.0
    h: float = (b - a) / float(n)
    result: float = (horner_eval(coeffs, a) + horner_eval(coeffs, b)) / 2.0
    for i in range(1, n):
        x: float = a + float(i) * h
        result += horner_eval(coeffs, x)
    return result * h


def simpson_rule(coeffs: List[float], a: float, b: float, n: int) -> float:
    """Approximate integral of polynomial using Simpson's 1/3 rule."""
    if n <= 0 or n % 2 != 0:
        return 0.0
    h: float = (b - a) / float(n)
    result: float = horner_eval(coeffs, a) + horner_eval(coeffs, b)
    for i in range(1, n):
        x: float = a + float(i) * h
        if i % 2 == 0:
            result += 2.0 * horner_eval(coeffs, x)
        else:
            result += 4.0 * horner_eval(coeffs, x)
    return result * h / 3.0


# --- 13. Linear interpolation, bezier curves ---


def lerp(a: float, b: float, t: float) -> float:
    """Linear interpolation between a and b."""
    return a + (b - a) * t


def bezier_quadratic(
    x0: float, y0: float, x1: float, y1: float,
    x2: float, y2: float, t: float,
) -> Tuple[float, float]:
    """Quadratic Bezier curve point at parameter t."""
    u: float = 1.0 - t
    bx: float = u * u * x0 + 2.0 * u * t * x1 + t * t * x2
    by: float = u * u * y0 + 2.0 * u * t * y1 + t * t * y2
    return (bx, by)


def bezier_cubic(
    x0: float, y0: float, x1: float, y1: float,
    x2: float, y2: float, x3: float, y3: float, t: float,
) -> Tuple[float, float]:
    """Cubic Bezier curve point at parameter t."""
    u: float = 1.0 - t
    u2: float = u * u
    u3: float = u2 * u
    t2: float = t * t
    t3: float = t2 * t
    bx: float = u3 * x0 + 3.0 * u2 * t * x1 + 3.0 * u * t2 * x2 + t3 * x3
    by: float = u3 * y0 + 3.0 * u2 * t * y1 + 3.0 * u * t2 * y2 + t3 * y3
    return (bx, by)


# --- 14. Angle calculations ---


def degrees_to_radians(deg: float) -> float:
    """Convert degrees to radians."""
    return deg * PI / 180.0


def radians_to_degrees(rad: float) -> float:
    """Convert radians to degrees."""
    return rad * 180.0 / PI


def angle_between_vectors(
    ax: float, ay: float, bx: float, by: float
) -> float:
    """Angle in radians between two 2D vectors."""
    dot: float = ax * bx + ay * by
    mag_a: float = newton_sqrt(ax * ax + ay * ay)
    mag_b: float = newton_sqrt(bx * bx + by * by)
    if mag_a < EPSILON or mag_b < EPSILON:
        return 0.0
    cos_val: float = dot / (mag_a * mag_b)
    return acos_approx(cos_val)


# --- 15. Coordinate transformations ---


def polar_to_cartesian(r: float, theta: float) -> Tuple[float, float]:
    """Convert polar coordinates to cartesian."""
    x: float = r * cos_approx(theta)
    y: float = r * sin_approx(theta)
    return (x, y)


def rotate_point(
    x: float, y: float, angle: float
) -> Tuple[float, float]:
    """Rotate a 2D point around the origin by angle (radians)."""
    cos_a: float = cos_approx(angle)
    sin_a: float = sin_approx(angle)
    rx: float = x * cos_a - y * sin_a
    ry: float = x * sin_a + y * cos_a
    return (rx, ry)


# --- Extra: Mixed int/float coercion patterns ---


def weighted_average(values: List[float], weights: List[int]) -> float:
    """Weighted average with int weights and float values."""
    total: float = 0.0
    weight_sum: int = 0
    n: int = len(values)
    for i in range(n):
        total += values[i] * float(weights[i])
        weight_sum += weights[i]
    if weight_sum == 0:
        return 0.0
    return total / float(weight_sum)


def int_to_float_distance(x1: int, y1: int, x2: int, y2: int) -> float:
    """Distance between integer-coordinate points returning float."""
    dx: float = float(x2 - x1)
    dy: float = float(y2 - y1)
    return newton_sqrt(dx * dx + dy * dy)


def grid_point_count(radius: int) -> int:
    """Count integer lattice points within a circle of given radius."""
    count: int = 0
    for x in range(-radius, radius + 1):
        for y in range(-radius, radius + 1):
            dist_sq: int = x * x + y * y
            if dist_sq <= radius * radius:
                count += 1
    return count


def normalize_int_vector(xs: List[int]) -> List[float]:
    """Normalize an integer vector to unit length (returns floats)."""
    sq_sum: float = 0.0
    for x in xs:
        sq_sum += float(x) * float(x)
    mag: float = newton_sqrt(sq_sum)
    if mag < EPSILON:
        result: List[float] = []
        for x in xs:
            result.append(float(x))
        return result
    result2: List[float] = []
    for x in xs:
        result2.append(float(x) / mag)
    return result2


def discrete_line_points(
    x1: int, y1: int, x2: int, y2: int
) -> List[Tuple[int, int]]:
    """Generate points on a line using Bresenham's algorithm."""
    points: List[Tuple[int, int]] = []
    dx: int = x2 - x1
    dy: int = y2 - y1
    sx: int = 1 if dx > 0 else -1
    sy: int = 1 if dy > 0 else -1
    if dx < 0:
        dx = -dx
    if dy < 0:
        dy = -dy
    cx: int = x1
    cy: int = y1
    if dx >= dy:
        err: int = dx // 2
        for _ in range(dx + 1):
            points.append((cx, cy))
            err -= dy
            if err < 0:
                cy += sy
                err += dx
            cx += sx
    else:
        err2: int = dy // 2
        for _ in range(dy + 1):
            points.append((cx, cy))
            err2 -= dx
            if err2 < 0:
                cx += sx
                err2 += dy
            cy += sy
    return points


# --- Test All ---


def test_all() -> bool:
    """Test every function in this module for correctness."""
    ok: bool = True

    # newton_sqrt
    if not float_eq(newton_sqrt(4.0), 2.0):
        ok = False
    if not float_eq(newton_sqrt(9.0), 3.0):
        ok = False
    if not float_eq(newton_sqrt(0.0), 0.0):
        ok = False

    # 1. distances
    if not float_eq(distance_2d(0.0, 0.0, 3.0, 4.0), 5.0):
        ok = False
    if not float_eq(distance_3d(0.0, 0.0, 0.0, 1.0, 2.0, 2.0), 3.0):
        ok = False
    if not float_eq(manhattan_distance(0.0, 0.0, 3.0, 4.0), 7.0):
        ok = False
    if not float_eq(chebyshev_distance(0.0, 0.0, 3.0, 4.0), 4.0):
        ok = False

    # 2. line intersection
    isect: Optional[Tuple[float, float]] = line_intersection(
        0.0, 0.0, 2.0, 2.0, 0.0, 2.0, 2.0, 0.0
    )
    if isect is None:
        ok = False
    else:
        if not float_eq(isect[0], 1.0) or not float_eq(isect[1], 1.0):
            ok = False
    # parallel lines should return None
    if line_intersection(0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 2.0) is not None:
        ok = False

    # 3. polygon area (unit square)
    sq_xs: List[float] = [0.0, 1.0, 1.0, 0.0]
    sq_ys: List[float] = [0.0, 0.0, 1.0, 1.0]
    if not float_eq(polygon_area(sq_xs, sq_ys), 1.0):
        ok = False

    # 4. convex hull area
    hull_xs: List[float] = [0.0, 1.0, 2.0, 1.0, 0.5]
    hull_ys: List[float] = [0.0, 0.0, 1.0, 2.0, 1.0]
    hull_a: float = convex_hull_area(hull_xs, hull_ys)
    if hull_a < 1.5 or hull_a > 4.5:
        ok = False

    # 5. point in polygon
    if not point_in_polygon(0.5, 0.5, sq_xs, sq_ys):
        ok = False
    if point_in_polygon(5.0, 5.0, sq_xs, sq_ys):
        ok = False

    # 6. triangle
    if not float_eq(triangle_area_coords(0.0, 0.0, 4.0, 0.0, 0.0, 3.0), 6.0):
        ok = False
    if triangle_classify(1.0, 1.0, 1.0) != "equilateral":
        ok = False
    if triangle_classify(3.0, 4.0, 5.0) != "right":
        ok = False
    if triangle_classify(2.0, 2.0, 3.5) != "obtuse":
        ok = False
    if not float_eq(heron_area(3.0, 4.0, 5.0), 6.0):
        ok = False

    # 7. circle
    if not float_eq(circle_area(1.0), PI):
        ok = False
    if not circles_intersect(0.0, 0.0, 1.0, 1.5, 0.0, 1.0):
        ok = False
    if circles_intersect(0.0, 0.0, 1.0, 10.0, 0.0, 1.0):
        ok = False

    # 8. matrix determinant
    if not float_eq(det_2x2(1.0, 2.0, 3.0, 4.0), -2.0):
        ok = False
    if not float_eq(det_3x3(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0), 1.0):
        ok = False

    # 9. vector operations
    if not float_eq(dot_product([1.0, 2.0, 3.0], [4.0, 5.0, 6.0]), 32.0):
        ok = False
    cp3: Tuple[float, float, float] = cross_product_3d(1.0, 0.0, 0.0, 0.0, 1.0, 0.0)
    if not float_eq(cp3[2], 1.0):
        ok = False
    vn: List[float] = vector_normalize([3.0, 4.0])
    if not float_eq(vn[0], 0.6) or not float_eq(vn[1], 0.8):
        ok = False

    # 11. Horner
    if not float_eq(horner_eval([2.0, 3.0, 1.0], 2.0), 15.0):
        ok = False

    # 12. numerical integration
    trap: float = trapezoidal_rule([1.0, 0.0], 0.0, 1.0, 1000)
    if float_abs(trap - 0.5) > 0.001:
        ok = False
    simp: float = simpson_rule([1.0, 0.0, 0.0], 0.0, 1.0, 100)
    if float_abs(simp - 1.0 / 3.0) > 0.001:
        ok = False

    # 13. interpolation, bezier
    if not float_eq(lerp(0.0, 10.0, 0.5), 5.0):
        ok = False
    bq0: Tuple[float, float] = bezier_quadratic(0.0, 0.0, 1.0, 2.0, 3.0, 0.0, 0.0)
    if not float_eq(bq0[0], 0.0) or not float_eq(bq0[1], 0.0):
        ok = False
    bq1: Tuple[float, float] = bezier_quadratic(0.0, 0.0, 1.0, 2.0, 3.0, 0.0, 1.0)
    if not float_eq(bq1[0], 3.0):
        ok = False
    bc0: Tuple[float, float] = bezier_cubic(0.0, 0.0, 1.0, 1.0, 2.0, 1.0, 3.0, 0.0, 0.0)
    if not float_eq(bc0[0], 0.0):
        ok = False

    # 14. angle calculations
    if not float_eq(degrees_to_radians(180.0), PI):
        ok = False
    if not float_eq(radians_to_degrees(PI), 180.0):
        ok = False
    abv: float = angle_between_vectors(1.0, 0.0, 0.0, 1.0)
    if float_abs(abv - PI / 2.0) > 0.01:
        ok = False

    # 15. coordinate transformations
    ptc: Tuple[float, float] = polar_to_cartesian(1.0, 0.0)
    if float_abs(ptc[0] - 1.0) > 0.01 or float_abs(ptc[1]) > 0.01:
        ok = False
    rp: Tuple[float, float] = rotate_point(1.0, 0.0, PI / 2.0)
    if float_abs(rp[0]) > 0.01 or float_abs(rp[1] - 1.0) > 0.01:
        ok = False

    # sin/cos basics
    if float_abs(sin_approx(0.0)) > 0.001:
        ok = False
    if float_abs(sin_approx(PI / 2.0) - 1.0) > 0.001:
        ok = False
    if float_abs(cos_approx(0.0) - 1.0) > 0.001:
        ok = False

    # mixed int/float coercion
    if not float_eq(weighted_average([1.0, 2.0, 3.0], [1, 1, 1]), 2.0):
        ok = False
    if not float_eq(weighted_average([10.0, 20.0], [1, 3]), 17.5):
        ok = False
    if not float_eq(int_to_float_distance(0, 0, 3, 4), 5.0):
        ok = False
    if grid_point_count(1) != 5:
        ok = False
    niv: List[float] = normalize_int_vector([3, 4])
    if float_abs(niv[0] - 0.6) > 0.01 or float_abs(niv[1] - 0.8) > 0.01:
        ok = False
    dlp: List[Tuple[int, int]] = discrete_line_points(0, 0, 3, 0)
    if len(dlp) != 4:
        ok = False

    return ok
