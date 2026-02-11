"""Real-world signal processing operations.

Mimics: scipy.signal, numpy convolution, DSP library patterns.
Implements convolution, moving average, peak detection, smoothing.
"""


def signal_moving_average(data: list[int], window: int) -> list[int]:
    """Compute moving average with given window size. Returns averaged * 100 for precision."""
    if len(data) == 0 or window <= 0:
        return []
    result: list[int] = []
    idx: int = 0
    while idx < len(data):
        start: int = idx - window // 2
        end: int = idx + window // 2 + 1
        if start < 0:
            start = 0
        if end > len(data):
            end = len(data)
        total: int = 0
        count: int = 0
        k: int = start
        while k < end:
            total = total + data[k]
            count = count + 1
            k = k + 1
        result.append((total * 100) // count)
        idx = idx + 1
    return result


def signal_convolve(data: list[int], kernel: list[int]) -> list[int]:
    """1D convolution of data with kernel. Output length = len(data)."""
    n: int = len(data)
    kn: int = len(kernel)
    half: int = kn // 2
    result: list[int] = []
    idx: int = 0
    while idx < n:
        total: int = 0
        ki: int = 0
        while ki < kn:
            data_idx: int = idx - half + ki
            if data_idx >= 0 and data_idx < n:
                total = total + data[data_idx] * kernel[ki]
            ki = ki + 1
        result.append(total)
        idx = idx + 1
    return result


def signal_find_peaks(data: list[int], min_height: int) -> list[int]:
    """Find peak indices where value is greater than both neighbors and above min_height."""
    peaks: list[int] = []
    if len(data) < 3:
        return peaks
    idx: int = 1
    while idx < len(data) - 1:
        if data[idx] > data[idx - 1] and data[idx] > data[idx + 1] and data[idx] >= min_height:
            peaks.append(idx)
        idx = idx + 1
    return peaks


def signal_find_valleys(data: list[int]) -> list[int]:
    """Find valley (local minimum) indices."""
    valleys: list[int] = []
    if len(data) < 3:
        return valleys
    idx: int = 1
    while idx < len(data) - 1:
        if data[idx] < data[idx - 1] and data[idx] < data[idx + 1]:
            valleys.append(idx)
        idx = idx + 1
    return valleys


def signal_differentiate(data: list[int]) -> list[int]:
    """Compute first difference (discrete derivative). Output length = len(data) - 1."""
    result: list[int] = []
    idx: int = 1
    while idx < len(data):
        result.append(data[idx] - data[idx - 1])
        idx = idx + 1
    return result


def signal_integrate(data: list[int]) -> list[int]:
    """Compute cumulative sum (discrete integral)."""
    result: list[int] = []
    total: int = 0
    idx: int = 0
    while idx < len(data):
        total = total + data[idx]
        result.append(total)
        idx = idx + 1
    return result


def signal_threshold(data: list[int], thresh: int) -> list[int]:
    """Apply threshold: values below thresh become 0, above stay."""
    result: list[int] = []
    idx: int = 0
    while idx < len(data):
        if data[idx] >= thresh:
            result.append(data[idx])
        else:
            result.append(0)
        idx = idx + 1
    return result


def signal_energy(data: list[int]) -> int:
    """Compute signal energy (sum of squares)."""
    total: int = 0
    idx: int = 0
    while idx < len(data):
        total = total + data[idx] * data[idx]
        idx = idx + 1
    return total


def signal_zero_crossings(data: list[int]) -> int:
    """Count zero crossings in signal."""
    if len(data) < 2:
        return 0
    count: int = 0
    idx: int = 1
    while idx < len(data):
        if (data[idx - 1] > 0 and data[idx] < 0) or (data[idx - 1] < 0 and data[idx] > 0):
            count = count + 1
        idx = idx + 1
    return count


def signal_clip(data: list[int], low: int, high: int) -> list[int]:
    """Clip signal values to [low, high] range."""
    result: list[int] = []
    idx: int = 0
    while idx < len(data):
        val: int = data[idx]
        if val < low:
            val = low
        if val > high:
            val = high
        result.append(val)
        idx = idx + 1
    return result


def test_module() -> int:
    """Test signal processing module."""
    passed: int = 0

    data: list[int] = [1, 3, 7, 1, 2, 8, 3, 1]

    # Test 1: moving average
    avg: list[int] = signal_moving_average([10, 20, 30, 40, 50], 3)
    if len(avg) == 5 and avg[1] == 2000:
        passed = passed + 1

    # Test 2: convolution with identity
    conv: list[int] = signal_convolve([1, 2, 3, 4], [0, 1, 0])
    if conv[1] == 2 and conv[2] == 3:
        passed = passed + 1

    # Test 3: find peaks
    peaks: list[int] = signal_find_peaks(data, 5)
    if len(peaks) == 2 and peaks[0] == 2 and peaks[1] == 5:
        passed = passed + 1

    # Test 4: find valleys
    valleys: list[int] = signal_find_valleys(data)
    if len(valleys) >= 1:
        passed = passed + 1

    # Test 5: differentiate
    diff: list[int] = signal_differentiate([1, 3, 6, 10])
    if diff[0] == 2 and diff[1] == 3 and diff[2] == 4:
        passed = passed + 1

    # Test 6: integrate
    integ: list[int] = signal_integrate([1, 2, 3, 4])
    if integ[3] == 10:
        passed = passed + 1

    # Test 7: energy
    eng: int = signal_energy([3, 4])
    if eng == 25:
        passed = passed + 1

    # Test 8: zero crossings
    zc: int = signal_zero_crossings([1, -1, 2, -2, 3])
    if zc == 4:
        passed = passed + 1

    return passed
