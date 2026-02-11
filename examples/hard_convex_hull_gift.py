"""Gift wrapping (Jarvis march) for convex hull on integer points."""


def cross_product(ox: int, oy: int, ax: int, ay: int, bx: int, by: int) -> int:
    """Compute cross product of vectors OA and OB."""
    result: int = (ax - ox) * (by - oy) - (ay - oy) * (bx - ox)
    return result


def dist_sq(ax: int, ay: int, bx: int, by: int) -> int:
    """Squared distance between two points."""
    dx: int = bx - ax
    dy: int = by - ay
    return dx * dx + dy * dy


def gift_wrap(xs: list[int], ys: list[int]) -> list[int]:
    """Compute convex hull using gift wrapping. Returns indices of hull points."""
    n: int = len(xs)
    if n < 3:
        result: list[int] = []
        i: int = 0
        while i < n:
            result.append(i)
            i = i + 1
        return result
    leftmost: int = 0
    i = 1
    while i < n:
        if xs[i] < xs[leftmost]:
            leftmost = i
        elif xs[i] == xs[leftmost] and ys[i] < ys[leftmost]:
            leftmost = i
        i = i + 1
    hull: list[int] = []
    current: int = leftmost
    done: int = 0
    while done == 0:
        hull.append(current)
        candidate: int = 0
        j: int = 1
        while j < n:
            if candidate == current:
                candidate = j
            else:
                cp: int = cross_product(xs[current], ys[current], xs[candidate], ys[candidate], xs[j], ys[j])
                if cp < 0:
                    candidate = j
                elif cp == 0:
                    d_cand: int = dist_sq(xs[current], ys[current], xs[candidate], ys[candidate])
                    d_j: int = dist_sq(xs[current], ys[current], xs[j], ys[j])
                    if d_j > d_cand:
                        candidate = j
            j = j + 1
        current = candidate
        if current == leftmost:
            done = 1
    return hull


def hull_area_twice(xs: list[int], ys: list[int], hull: list[int]) -> int:
    """Compute twice the area of the convex hull using shoelace formula."""
    h: int = len(hull)
    area: int = 0
    i: int = 0
    while i < h:
        j2: int = (i + 1) % h
        hi: int = hull[i]
        hj: int = hull[j2]
        area = area + xs[hi] * ys[hj] - xs[hj] * ys[hi]
        i = i + 1
    if area < 0:
        area = 0 - area
    return area


def test_module() -> int:
    """Test convex hull."""
    passed: int = 0

    xs1: list[int] = [0, 1, 2, 1]
    ys1: list[int] = [0, 0, 1, 1]
    hull1: list[int] = gift_wrap(xs1, ys1)
    if len(hull1) == 3 or len(hull1) == 4:
        passed = passed + 1

    xs2: list[int] = [0, 4, 4, 0, 2]
    ys2: list[int] = [0, 0, 4, 4, 2]
    hull2: list[int] = gift_wrap(xs2, ys2)
    if len(hull2) == 4:
        passed = passed + 1

    area2: int = hull_area_twice(xs2, ys2, hull2)
    if area2 == 32:
        passed = passed + 1

    xs3: list[int] = [0, 1]
    ys3: list[int] = [0, 1]
    hull3: list[int] = gift_wrap(xs3, ys3)
    if len(hull3) == 2:
        passed = passed + 1

    if cross_product(0, 0, 1, 0, 0, 1) == 1:
        passed = passed + 1

    if dist_sq(0, 0, 3, 4) == 25:
        passed = passed + 1

    return passed
