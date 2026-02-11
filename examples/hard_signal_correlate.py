def cross_correlate(sig_a: list[float], sig_b: list[float]) -> list[float]:
    na: int = len(sig_a)
    nb: int = len(sig_b)
    sum_len: int = na + nb
    out_len: int = sum_len - 1
    result: list[float] = []
    lag: int = 0
    while lag < out_len:
        total: float = 0.0
        j: int = 0
        while j < na:
            bj: int = lag - j
            if bj >= 0 and bj < nb:
                total = total + sig_a[j] * sig_b[bj]
            j = j + 1
        result.append(total)
        lag = lag + 1
    return result

def autocorrelate(signal: list[float]) -> list[float]:
    return cross_correlate(signal, signal)

def mean_of(data: list[float]) -> float:
    n: int = len(data)
    total: float = 0.0
    i: int = 0
    while i < n:
        total = total + data[i]
        i = i + 1
    return total / (n * 1.0)

def covariance(sig_a: list[float], sig_b: list[float]) -> float:
    n: int = len(sig_a)
    ma: float = mean_of(sig_a)
    mb: float = mean_of(sig_b)
    total: float = 0.0
    i: int = 0
    while i < n:
        total = total + (sig_a[i] - ma) * (sig_b[i] - mb)
        i = i + 1
    return total / (n * 1.0)

def pearson_correlation(sig_a: list[float], sig_b: list[float]) -> float:
    cov: float = covariance(sig_a, sig_b)
    va: float = covariance(sig_a, sig_a)
    vb: float = covariance(sig_b, sig_b)
    denom: float = (va * vb) ** 0.5
    if denom == 0.0:
        return 0.0
    return cov / denom

def test_module() -> int:
    passed: int = 0
    a: list[float] = [1.0, 0.0, 0.0]
    b: list[float] = [1.0, 0.0, 0.0]
    cc: list[float] = cross_correlate(a, b)
    cc0: float = cc[0]
    if cc0 == 1.0:
        passed = passed + 1
    nc: int = len(cc)
    if nc == 5:
        passed = passed + 1
    ac: list[float] = autocorrelate(a)
    ac0: float = ac[0]
    if ac0 == 1.0:
        passed = passed + 1
    m: float = mean_of(a)
    diff: float = m - 0.333333
    if diff < 0.01 and diff > (0.0 - 0.01):
        passed = passed + 1
    x: list[float] = [1.0, 2.0, 3.0]
    y: list[float] = [2.0, 4.0, 6.0]
    pc: float = pearson_correlation(x, y)
    diff2: float = pc - 1.0
    if diff2 < 0.01 and diff2 > (0.0 - 0.01):
        passed = passed + 1
    return passed
