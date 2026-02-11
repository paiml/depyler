def cross_2d(ox: int, oy: int, ax: int, ay: int, bx: int, by: int) -> int:
    return (ax - ox) * (by - oy) - (ay - oy) * (bx - ox)


def find_leftmost(xs: list[int], ys: list[int]) -> int:
    idx: int = 0
    i: int = 1
    while i < len(xs):
        if xs[i] < xs[idx]:
            idx = i
        elif xs[i] == xs[idx] and ys[i] < ys[idx]:
            idx = i
        i = i + 1
    return idx


def convex_hull_jarvis(xs: list[int], ys: list[int]) -> list[int]:
    n: int = len(xs)
    if n < 3:
        result: list[int] = []
        i: int = 0
        while i < n:
            result.append(i)
            i = i + 1
        return result
    hull: list[int] = []
    start: int = find_leftmost(xs, ys)
    p: int = start
    done: int = 0
    while done == 0:
        hull.append(p)
        q: int = 0
        if p == 0:
            q = 1
        i: int = 0
        while i < n:
            if i != p:
                cp: int = cross_2d(xs[p], ys[p], xs[q], ys[q], xs[i], ys[i])
                if cp < 0:
                    q = i
                elif cp == 0:
                    dx1: int = xs[q] - xs[p]
                    dy1: int = ys[q] - ys[p]
                    dx2: int = xs[i] - xs[p]
                    dy2: int = ys[i] - ys[p]
                    d1: int = dx1 * dx1 + dy1 * dy1
                    d2: int = dx2 * dx2 + dy2 * dy2
                    if d2 > d1:
                        q = i
            i = i + 1
        p = q
        if p == start:
            done = 1
    return hull


def hull_size(xs: list[int], ys: list[int]) -> int:
    h: list[int] = convex_hull_jarvis(xs, ys)
    return len(h)


def test_module() -> int:
    passed: int = 0
    xs: list[int] = [0, 1, 2, 1]
    ys: list[int] = [0, 1, 0, -1]
    h: list[int] = convex_hull_jarvis(xs, ys)
    if len(h) == 4:
        passed = passed + 1
    if h[0] == 0:
        passed = passed + 1
    xs2: list[int] = [0, 4, 4, 0, 2]
    ys2: list[int] = [0, 0, 4, 4, 2]
    h2: list[int] = convex_hull_jarvis(xs2, ys2)
    if len(h2) == 4:
        passed = passed + 1
    if hull_size([0, 1], [0, 1]) == 2:
        passed = passed + 1
    if hull_size([0, 1, 0], [0, 0, 1]) == 3:
        passed = passed + 1
    return passed
