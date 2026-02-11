def simple_moving_avg_n(data: list[float], n_window: int) -> list[float]:
    result: list[float] = []
    n: int = len(data)
    i: int = 0
    while i < n:
        if i + 1 < n_window:
            result.append(data[i])
        else:
            total: float = 0.0
            j: int = 0
            while j < n_window:
                idx: int = i - j
                total = total + data[idx]
                j = j + 1
            denom: float = n_window * 1.0
            result.append(total / denom)
        i = i + 1
    return result

def weighted_moving_avg(data: list[float], weights: list[float]) -> list[float]:
    result: list[float] = []
    n: int = len(data)
    w: int = len(weights)
    w_thresh: int = w - 1
    i: int = 0
    while i < n:
        if i < w_thresh:
            result.append(data[i])
        else:
            total: float = 0.0
            wsum: float = 0.0
            j: int = 0
            while j < w:
                idx: int = i - j
                total = total + data[idx] * weights[j]
                wsum = wsum + weights[j]
                j = j + 1
            result.append(total / wsum)
        i = i + 1
    return result

def exponential_moving_avg(data: list[float], alpha: float) -> list[float]:
    result: list[float] = [data[0]]
    n: int = len(data)
    i: int = 1
    while i < n:
        prev: float = result[i - 1]
        val: float = alpha * data[i] + (1.0 - alpha) * prev
        result.append(val)
        i = i + 1
    return result

def cumulative_avg(data: list[float]) -> list[float]:
    result: list[float] = []
    total: float = 0.0
    n: int = len(data)
    i: int = 0
    while i < n:
        total = total + data[i]
        result.append(total / ((i + 1) * 1.0))
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    d: list[float] = [1.0, 2.0, 3.0, 4.0, 5.0]
    r: list[float] = simple_moving_avg_n(d, 3)
    v: float = r[2]
    if v == 2.0:
        passed = passed + 1
    e: list[float] = exponential_moving_avg(d, 1.0)
    e2: float = e[1]
    if e2 == 2.0:
        passed = passed + 1
    c: list[float] = cumulative_avg(d)
    c0: float = c[0]
    if c0 == 1.0:
        passed = passed + 1
    c4: float = c[4]
    if c4 == 3.0:
        passed = passed + 1
    w: list[float] = [1.0, 1.0]
    wr: list[float] = weighted_moving_avg(d, w)
    n: int = len(wr)
    if n == 5:
        passed = passed + 1
    return passed
