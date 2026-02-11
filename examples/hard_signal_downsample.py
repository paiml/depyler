def downsample_by2(data: list[float]) -> list[float]:
    result: list[float] = []
    n: int = len(data)
    i: int = 0
    while i < n:
        result.append(data[i])
        i = i + 2
    return result

def downsample_by3(data: list[float]) -> list[float]:
    result: list[float] = []
    n: int = len(data)
    i: int = 0
    while i < n:
        result.append(data[i])
        i = i + 3
    return result

def upsample_zero2(data: list[float]) -> list[float]:
    result: list[float] = []
    n: int = len(data)
    i: int = 0
    while i < n:
        result.append(data[i])
        result.append(0.0)
        i = i + 1
    return result

def average_pool2(data: list[float]) -> list[float]:
    result: list[float] = []
    n: int = len(data)
    i: int = 0
    while i + 1 < n:
        a: float = data[i]
        next_i: int = i + 1
        b: float = data[next_i]
        result.append((a + b) / 2.0)
        i = i + 2
    return result

def max_pool2(data: list[float]) -> list[float]:
    result: list[float] = []
    n: int = len(data)
    i: int = 0
    while i + 1 < n:
        a: float = data[i]
        next_i: int = i + 1
        b: float = data[next_i]
        mx: float = a
        if b > a:
            mx = b
        result.append(mx)
        i = i + 2
    return result

def decimate_simple(data: list[float]) -> list[float]:
    n: int = len(data)
    filtered: list[float] = []
    i: int = 0
    while i < n:
        total: float = 0.0
        cnt: int = 0
        j: int = i - 1
        while j <= i + 1:
            if j >= 0 and j < n:
                total = total + data[j]
                cnt = cnt + 1
            j = j + 1
        filtered.append(total / (cnt * 1.0))
        i = i + 1
    return downsample_by2(filtered)

def test_module() -> int:
    passed: int = 0
    d: list[float] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
    ds: list[float] = downsample_by2(d)
    n: int = len(ds)
    if n == 3:
        passed = passed + 1
    ds0: float = ds[0]
    if ds0 == 1.0:
        passed = passed + 1
    us: list[float] = upsample_zero2(d)
    nu: int = len(us)
    if nu == 12:
        passed = passed + 1
    ap: list[float] = average_pool2(d)
    ap0: float = ap[0]
    if ap0 == 1.5:
        passed = passed + 1
    mp: list[float] = max_pool2(d)
    mp0: float = mp[0]
    if mp0 == 2.0:
        passed = passed + 1
    return passed
