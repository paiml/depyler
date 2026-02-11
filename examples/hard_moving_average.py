# Simple, exponential, and weighted moving average using integer-scaled arithmetic


def simple_moving_average(data: list[int], window: int) -> list[int]:
    # Returns SMA values (each is data_value * 1, integer division)
    result: list[int] = []
    if window <= 0 or len(data) < window:
        return result
    i: int = 0
    while i <= len(data) - window:
        total: int = 0
        j: int = i
        while j < i + window:
            total = total + data[j]
            j = j + 1
        result.append(total // window)
        i = i + 1
    return result


def exponential_moving_average(data: list[int], alpha_num: int, alpha_den: int) -> list[int]:
    # EMA with alpha = alpha_num / alpha_den
    # Returns values scaled by alpha_den for precision
    if len(data) == 0:
        return []
    result: list[int] = []
    ema: int = data[0] * alpha_den
    result.append(data[0])
    i: int = 1
    while i < len(data):
        ema = (alpha_num * data[i] * alpha_den + (alpha_den - alpha_num) * ema) // alpha_den
        result.append(ema // alpha_den)
        i = i + 1
    return result


def weighted_moving_average(data: list[int], weights: list[int]) -> list[int]:
    # WMA with given weights
    window: int = len(weights)
    result: list[int] = []
    if window == 0 or len(data) < window:
        return result
    weight_sum: int = 0
    k: int = 0
    while k < window:
        weight_sum = weight_sum + weights[k]
        k = k + 1
    i: int = 0
    while i <= len(data) - window:
        total: int = 0
        j: int = 0
        while j < window:
            total = total + data[i + j] * weights[j]
            j = j + 1
        if weight_sum > 0:
            result.append(total // weight_sum)
        else:
            result.append(0)
        i = i + 1
    return result


def cumulative_average(data: list[int]) -> list[int]:
    result: list[int] = []
    total: int = 0
    i: int = 0
    while i < len(data):
        total = total + data[i]
        result.append(total // (i + 1))
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0

    # Test 1: SMA of [1,2,3,4,5] with window 3
    data: list[int] = [1, 2, 3, 4, 5]
    sma: list[int] = simple_moving_average(data, 3)
    if sma[0] == 2 and sma[1] == 3 and sma[2] == 4:
        passed = passed + 1

    # Test 2: SMA length
    if len(sma) == 3:
        passed = passed + 1

    # Test 3: SMA of constant
    const: list[int] = [5, 5, 5, 5]
    sma2: list[int] = simple_moving_average(const, 2)
    if sma2[0] == 5 and sma2[1] == 5 and sma2[2] == 5:
        passed = passed + 1

    # Test 4: EMA first value equals data
    ema: list[int] = exponential_moving_average(data, 1, 2)
    if ema[0] == 1:
        passed = passed + 1

    # Test 5: EMA length
    if len(ema) == 5:
        passed = passed + 1

    # Test 6: weighted MA with equal weights = SMA
    weights: list[int] = [1, 1, 1]
    wma: list[int] = weighted_moving_average(data, weights)
    if wma[0] == 2 and wma[1] == 3 and wma[2] == 4:
        passed = passed + 1

    # Test 7: cumulative average
    ca: list[int] = cumulative_average(data)
    # [1, 1, 2, 2, 3]
    if ca[0] == 1 and ca[4] == 3:
        passed = passed + 1

    return passed
