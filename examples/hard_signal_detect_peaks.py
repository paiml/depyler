def find_peaks(data: list[float]) -> list[int]:
    peaks: list[int] = []
    n: int = len(data)
    i: int = 1
    while i < n - 1:
        prev: float = data[i - 1]
        curr: float = data[i]
        next_i: int = i + 1
        nxt: float = data[next_i]
        if curr > prev and curr > nxt:
            peaks.append(i)
        i = i + 1
    return peaks

def find_valleys(data: list[float]) -> list[int]:
    valleys: list[int] = []
    n: int = len(data)
    i: int = 1
    while i < n - 1:
        prev: float = data[i - 1]
        curr: float = data[i]
        next_i: int = i + 1
        nxt: float = data[next_i]
        if curr < prev and curr < nxt:
            valleys.append(i)
        i = i + 1
    return valleys

def zero_crossings(data: list[float]) -> int:
    count: int = 0
    n: int = len(data)
    i: int = 1
    while i < n:
        prev: float = data[i - 1]
        curr: float = data[i]
        if (prev >= 0.0 and curr < 0.0) or (prev < 0.0 and curr >= 0.0):
            count = count + 1
        i = i + 1
    return count

def threshold_crossings(data: list[float], threshold: float) -> int:
    count: int = 0
    n: int = len(data)
    i: int = 1
    while i < n:
        prev: float = data[i - 1]
        curr: float = data[i]
        if (prev < threshold and curr >= threshold) or (prev >= threshold and curr < threshold):
            count = count + 1
        i = i + 1
    return count

def peak_prominence(data: list[float], peak_idx: int) -> float:
    peak_val: float = data[peak_idx]
    left_min: float = peak_val
    i: int = peak_idx - 1
    while i >= 0:
        v: float = data[i]
        if v < left_min:
            left_min = v
        i = i - 1
    right_min: float = peak_val
    n: int = len(data)
    j: int = peak_idx + 1
    while j < n:
        v2: float = data[j]
        if v2 < right_min:
            right_min = v2
        j = j + 1
    higher_min: float = left_min
    if right_min > higher_min:
        higher_min = right_min
    return peak_val - higher_min

def test_module() -> int:
    passed: int = 0
    d: list[float] = [0.0, 3.0, 1.0, 5.0, 2.0]
    p: list[int] = find_peaks(d)
    np: int = len(p)
    if np == 2:
        passed = passed + 1
    v: list[int] = find_valleys(d)
    nv: int = len(v)
    if nv == 1:
        passed = passed + 1
    d2: list[float] = [1.0, (0.0 - 1.0), 1.0, (0.0 - 1.0)]
    zc: int = zero_crossings(d2)
    if zc == 3:
        passed = passed + 1
    tc: int = threshold_crossings(d, 2.5)
    if tc >= 2:
        passed = passed + 1
    prom: float = peak_prominence(d, 1)
    if prom == 2.0:
        passed = passed + 1
    return passed
