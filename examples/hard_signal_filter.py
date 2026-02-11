def low_pass_filter(data: list[float], alpha: float) -> list[float]:
    result: list[float] = [data[0]]
    n: int = len(data)
    i: int = 1
    while i < n:
        prev: float = result[i - 1]
        val: float = alpha * data[i] + (1.0 - alpha) * prev
        result.append(val)
        i = i + 1
    return result

def high_pass_filter(data: list[float], alpha: float) -> list[float]:
    result: list[float] = [data[0]]
    n: int = len(data)
    i: int = 1
    while i < n:
        prev_out: float = result[i - 1]
        val: float = alpha * (prev_out + data[i] - data[i - 1])
        result.append(val)
        i = i + 1
    return result

def band_pass(data: list[float], low_a: float, high_a: float) -> list[float]:
    lp: list[float] = low_pass_filter(data, low_a)
    hp: list[float] = high_pass_filter(lp, high_a)
    return hp

def median_filter(data: list[float], window: int) -> list[float]:
    result: list[float] = []
    n: int = len(data)
    i: int = 0
    while i < n:
        vals: list[float] = []
        j: int = i - window
        while j <= i + window:
            if j >= 0 and j < n:
                vals.append(data[j])
            j = j + 1
        vn: int = len(vals)
        k: int = 0
        while k < vn - 1:
            m: int = k + 1
            while m < vn:
                vk: float = vals[k]
                vm: float = vals[m]
                if vk > vm:
                    vals[k] = vm
                    vals[m] = vk
                m = m + 1
            k = k + 1
        mid: int = vn // 2
        result.append(vals[mid])
        i = i + 1
    return result

def test_module() -> int:
    passed: int = 0
    d: list[float] = [1.0, 2.0, 3.0, 4.0, 5.0]
    lp: list[float] = low_pass_filter(d, 1.0)
    lp2: float = lp[1]
    if lp2 == 2.0:
        passed = passed + 1
    lp0: float = lp[0]
    if lp0 == 1.0:
        passed = passed + 1
    hp: list[float] = high_pass_filter(d, 1.0)
    hp0: float = hp[0]
    if hp0 == 1.0:
        passed = passed + 1
    d2: list[float] = [1.0, 100.0, 1.0, 1.0, 1.0]
    mf: list[float] = median_filter(d2, 1)
    mf1: float = mf[1]
    if mf1 == 1.0:
        passed = passed + 1
    bp: list[float] = band_pass(d, 0.5, 0.5)
    n: int = len(bp)
    if n == 5:
        passed = passed + 1
    return passed
