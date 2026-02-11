def linear_interp(x0: float, y0: float, x1: float, y1: float, x: float) -> float:
    if x1 == x0:
        return y0
    t: float = (x - x0) / (x1 - x0)
    return y0 + t * (y1 - y0)

def nearest_interp(xs: list[float], ys: list[float], x: float) -> float:
    n: int = len(xs)
    best_idx: int = 0
    best_dist: float = 1.0e30
    i: int = 0
    while i < n:
        d: float = x - xs[i]
        if d < 0.0:
            d = 0.0 - d
        if d < best_dist:
            best_dist = d
            best_idx = i
        i = i + 1
    return ys[best_idx]

def piecewise_linear(xs: list[float], ys: list[float], x: float) -> float:
    n: int = len(xs)
    if x <= xs[0]:
        return ys[0]
    last: int = n - 1
    xlast: float = xs[last]
    if x >= xlast:
        return ys[last]
    i: int = 0
    while i < n - 1:
        next_i: int = i + 1
        xi: float = xs[i]
        xn: float = xs[next_i]
        if x >= xi and x <= xn:
            return linear_interp(xi, ys[i], xn, ys[next_i], x)
        i = i + 1
    return ys[last]

def upsample_linear(data: list[float], factor: int) -> list[float]:
    result: list[float] = []
    n: int = len(data)
    i: int = 0
    while i < n - 1:
        result.append(data[i])
        j: int = 1
        while j < factor:
            t: float = j * 1.0 / (factor * 1.0)
            next_i: int = i + 1
            val: float = data[i] + t * (data[next_i] - data[i])
            result.append(val)
            j = j + 1
        i = i + 1
    last: int = n - 1
    result.append(data[last])
    return result

def test_module() -> int:
    passed: int = 0
    v: float = linear_interp(0.0, 0.0, 10.0, 10.0, 5.0)
    if v == 5.0:
        passed = passed + 1
    v2: float = linear_interp(0.0, 0.0, 10.0, 10.0, 0.0)
    if v2 == 0.0:
        passed = passed + 1
    xs: list[float] = [0.0, 5.0, 10.0]
    ys: list[float] = [0.0, 50.0, 100.0]
    nr: float = nearest_interp(xs, ys, 2.0)
    if nr == 0.0:
        passed = passed + 1
    pw: float = piecewise_linear(xs, ys, 2.5)
    if pw == 25.0:
        passed = passed + 1
    d: list[float] = [0.0, 10.0]
    up: list[float] = upsample_linear(d, 2)
    up1: float = up[1]
    if up1 == 5.0:
        passed = passed + 1
    return passed
