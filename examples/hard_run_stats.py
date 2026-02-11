# Running mean, variance, min, max using integer-scaled arithmetic


def running_mean(data: list[int], scale: int) -> list[int]:
    # Returns cumulative mean at each step, scaled
    result: list[int] = []
    total: int = 0
    i: int = 0
    while i < len(data):
        total = total + data[i]
        result.append(total * scale // (i + 1))
        i = i + 1
    return result


def running_min(data: list[int]) -> list[int]:
    result: list[int] = []
    if len(data) == 0:
        return result
    current_min: int = data[0]
    result.append(current_min)
    i: int = 1
    while i < len(data):
        if data[i] < current_min:
            current_min = data[i]
        result.append(current_min)
        i = i + 1
    return result


def running_max(data: list[int]) -> list[int]:
    result: list[int] = []
    if len(data) == 0:
        return result
    current_max: int = data[0]
    result.append(current_max)
    i: int = 1
    while i < len(data):
        if data[i] > current_max:
            current_max = data[i]
        result.append(current_max)
        i = i + 1
    return result


def running_variance(data: list[int], scale: int) -> list[int]:
    # Welford's online algorithm, returns variance * scale at each step
    result: list[int] = []
    if len(data) == 0:
        return result
    mean_scaled: int = data[0] * scale
    m2: int = 0
    result.append(0)
    i: int = 1
    while i < len(data):
        n: int = i + 1
        delta: int = data[i] * scale - mean_scaled
        mean_scaled = mean_scaled + delta // n
        delta2: int = data[i] * scale - mean_scaled
        m2 = m2 + delta * delta2 // scale
        result.append(m2 // n)
        i = i + 1
    return result


def running_range(data: list[int]) -> list[int]:
    mins: list[int] = running_min(data)
    maxs: list[int] = running_max(data)
    result: list[int] = []
    i: int = 0
    while i < len(mins):
        result.append(maxs[i] - mins[i])
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0
    scale: int = 1000

    data: list[int] = [10, 20, 30, 40, 50]

    # Test 1: running mean first value
    rm: list[int] = running_mean(data, scale)
    if rm[0] == 10000:
        passed = passed + 1

    # Test 2: running mean after all values
    if rm[4] == 30000:
        passed = passed + 1

    # Test 3: running min
    rmin: list[int] = running_min(data)
    if rmin[0] == 10 and rmin[4] == 10:
        passed = passed + 1

    # Test 4: running max
    rmax: list[int] = running_max(data)
    if rmax[0] == 10 and rmax[4] == 50:
        passed = passed + 1

    # Test 5: running range
    rng: list[int] = running_range(data)
    if rng[0] == 0 and rng[4] == 40:
        passed = passed + 1

    # Test 6: running variance of constant = 0
    const: list[int] = [5, 5, 5, 5]
    rv: list[int] = running_variance(const, scale)
    if rv[3] == 0:
        passed = passed + 1

    # Test 7: running min with decreasing data
    dec: list[int] = [50, 40, 30, 20, 10]
    rmin2: list[int] = running_min(dec)
    if rmin2[0] == 50 and rmin2[2] == 30 and rmin2[4] == 10:
        passed = passed + 1

    return passed
