def min_max_normalize(data: list[float]) -> list[float]:
    n: int = len(data)
    mn: float = data[0]
    mx: float = data[0]
    i: int = 1
    while i < n:
        v: float = data[i]
        if v < mn:
            mn = v
        if v > mx:
            mx = v
        i = i + 1
    result: list[float] = []
    rng: float = mx - mn
    if rng == 0.0:
        j: int = 0
        while j < n:
            result.append(0.0)
            j = j + 1
        return result
    j2: int = 0
    while j2 < n:
        result.append((data[j2] - mn) / rng)
        j2 = j2 + 1
    return result

def z_score_normalize(data: list[float]) -> list[float]:
    n: int = len(data)
    total: float = 0.0
    i: int = 0
    while i < n:
        total = total + data[i]
        i = i + 1
    mean: float = total / (n * 1.0)
    var_sum: float = 0.0
    i2: int = 0
    while i2 < n:
        diff: float = data[i2] - mean
        var_sum = var_sum + diff * diff
        i2 = i2 + 1
    std: float = (var_sum / (n * 1.0)) ** 0.5
    result: list[float] = []
    if std == 0.0:
        j: int = 0
        while j < n:
            result.append(0.0)
            j = j + 1
        return result
    j2: int = 0
    while j2 < n:
        result.append((data[j2] - mean) / std)
        j2 = j2 + 1
    return result

def decimal_scaling(data: list[float]) -> list[float]:
    n: int = len(data)
    mx: float = 0.0
    i: int = 0
    while i < n:
        v: float = data[i]
        if v < 0.0:
            v = 0.0 - v
        if v > mx:
            mx = v
        i = i + 1
    scale: float = 1.0
    while scale <= mx:
        scale = scale * 10.0
    result: list[float] = []
    j: int = 0
    while j < n:
        result.append(data[j] / scale)
        j = j + 1
    return result

def test_module() -> int:
    passed: int = 0
    d: list[float] = [0.0, 5.0, 10.0]
    mm: list[float] = min_max_normalize(d)
    mm0: float = mm[0]
    if mm0 == 0.0:
        passed = passed + 1
    mm2: float = mm[2]
    if mm2 == 1.0:
        passed = passed + 1
    z: list[float] = z_score_normalize(d)
    z1: float = z[1]
    diff: float = z1 - 0.0
    if diff < 0.01 and diff > (0.0 - 0.01):
        passed = passed + 1
    ds: list[float] = decimal_scaling(d)
    ds0: float = ds[0]
    if ds0 == 0.0:
        passed = passed + 1
    d2: list[float] = [5.0, 5.0, 5.0]
    mm3: list[float] = min_max_normalize(d2)
    mm30: float = mm3[0]
    if mm30 == 0.0:
        passed = passed + 1
    return passed
