def exponential_growth(p0: float, rate: float, steps: int) -> float:
    p: float = p0
    i: int = 0
    while i < steps:
        p = p + p * rate
        i = i + 1
    return p

def logistic_growth(p0: float, rate: float, capacity: float, steps: int) -> float:
    p: float = p0
    i: int = 0
    while i < steps:
        growth: float = rate * p * (1.0 - p / capacity)
        p = p + growth
        i = i + 1
    return p

def doubling_time(rate: float) -> float:
    if rate <= 0.0:
        return 0.0
    return 0.693147 / rate

def harvest_model(p0: float, rate: float, harvest: float, steps: int) -> float:
    p: float = p0
    i: int = 0
    while i < steps:
        p = p + p * rate - harvest
        if p < 0.0:
            p = 0.0
        i = i + 1
    return p

def population_series(p0: float, rate: float, steps: int) -> list[float]:
    result: list[float] = [p0]
    p: float = p0
    i: int = 0
    while i < steps:
        p = p + p * rate
        result.append(p)
        i = i + 1
    return result

def carrying_capacity_time(p0: float, rate: float, capacity: float, threshold: float) -> float:
    p: float = p0
    t: float = 0.0
    while p < capacity * threshold and t < 10000.0:
        growth: float = rate * p * (1.0 - p / capacity)
        p = p + growth
        t = t + 1.0
    return t

def test_module() -> int:
    passed: int = 0
    p1: float = exponential_growth(100.0, 0.1, 1)
    if p1 == 110.0:
        passed = passed + 1
    p2: float = logistic_growth(100.0, 0.1, 1000.0, 1)
    diff: float = p2 - 109.0
    if diff < 0.1 and diff > (0.0 - 0.1):
        passed = passed + 1
    dt: float = doubling_time(0.1)
    diff2: float = dt - 6.93147
    if diff2 < 0.01 and diff2 > (0.0 - 0.01):
        passed = passed + 1
    h: float = harvest_model(100.0, 0.5, 200.0, 1)
    if h == 0.0:
        passed = passed + 1
    s: list[float] = population_series(100.0, 0.0, 3)
    n: int = len(s)
    if n == 4:
        passed = passed + 1
    return passed
