def polygon_area_2x(xs: list[int], ys: list[int]) -> int:
    n: int = len(xs)
    area: int = 0
    i: int = 0
    while i < n:
        j: int = (i + 1) % n
        area = area + xs[i] * ys[j]
        area = area - xs[j] * ys[i]
        i = i + 1
    if area < 0:
        area = -area
    return area


def polygon_perimeter_squared_sum(xs: list[int], ys: list[int]) -> int:
    n: int = len(xs)
    total: int = 0
    i: int = 0
    while i < n:
        j: int = (i + 1) % n
        dx: int = xs[j] - xs[i]
        dy: int = ys[j] - ys[i]
        total = total + dx * dx + dy * dy
        i = i + 1
    return total


def polygon_centroid_nx(xs: list[int], ys: list[int]) -> int:
    n: int = len(xs)
    cx: int = 0
    i: int = 0
    while i < n:
        cx = cx + xs[i]
        i = i + 1
    return cx


def polygon_centroid_ny(xs: list[int], ys: list[int]) -> int:
    n: int = len(xs)
    cy: int = 0
    i: int = 0
    while i < n:
        cy = cy + ys[i]
        i = i + 1
    return cy


def is_convex(xs: list[int], ys: list[int]) -> int:
    n: int = len(xs)
    if n < 3:
        return 0
    sign: int = 0
    i: int = 0
    while i < n:
        j: int = (i + 1) % n
        k: int = (i + 2) % n
        cp: int = (xs[j] - xs[i]) * (ys[k] - ys[j]) - (ys[j] - ys[i]) * (xs[k] - xs[j])
        if cp != 0:
            if sign == 0:
                if cp > 0:
                    sign = 1
                else:
                    sign = -1
            else:
                if (cp > 0 and sign == -1) or (cp < 0 and sign == 1):
                    return 0
        i = i + 1
    return 1


def test_module() -> int:
    passed: int = 0
    xs: list[int] = [0, 4, 4, 0]
    ys: list[int] = [0, 0, 3, 3]
    if polygon_area_2x(xs, ys) == 24:
        passed = passed + 1
    tri_xs: list[int] = [0, 4, 0]
    tri_ys: list[int] = [0, 0, 3]
    if polygon_area_2x(tri_xs, tri_ys) == 12:
        passed = passed + 1
    if polygon_centroid_nx([0, 3, 6], [0, 0, 0]) == 9:
        passed = passed + 1
    if polygon_centroid_ny([0, 0, 0], [0, 3, 6]) == 9:
        passed = passed + 1
    if is_convex([0, 4, 4, 0], [0, 0, 3, 3]) == 1:
        passed = passed + 1
    ps: int = polygon_perimeter_squared_sum([0, 3, 3, 0], [0, 0, 4, 4])
    if ps == 50:
        passed = passed + 1
    return passed
