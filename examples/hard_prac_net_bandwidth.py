"""Bandwidth estimation and measurement simulation.

Implements moving average bandwidth measurement, peak detection,
and utilization tracking over time windows.
"""


def bw_init_zeros(size: int) -> list[int]:
    """Initialize with zeros."""
    result: list[int] = []
    i: int = 0
    while i < size:
        result.append(0)
        i = i + 1
    return result


def bw_record_sample(samples: list[int], head_arr: list[int],
                     window_size: int, value: int) -> int:
    """Record a bandwidth sample in circular buffer. Returns new head."""
    h: int = head_arr[0]
    samples[h] = value
    new_h: int = (h + 1) % window_size
    head_arr[0] = new_h
    return new_h


def bw_moving_average(samples: list[int], window_size: int) -> int:
    """Compute moving average of samples."""
    total: int = 0
    i: int = 0
    while i < window_size:
        s: int = samples[i]
        total = total + s
        i = i + 1
    return total // window_size


def bw_peak(samples: list[int], window_size: int) -> int:
    """Find peak bandwidth in window."""
    mx: int = samples[0]
    i: int = 1
    while i < window_size:
        s: int = samples[i]
        if s > mx:
            mx = s
        i = i + 1
    return mx


def bw_minimum(samples: list[int], window_size: int) -> int:
    """Find minimum bandwidth in window."""
    mn: int = samples[0]
    i: int = 1
    while i < window_size:
        s: int = samples[i]
        if s < mn:
            mn = s
        i = i + 1
    return mn


def bw_utilization_pct(actual: int, capacity: int) -> int:
    """Compute utilization percentage."""
    if capacity == 0:
        return 0
    return (actual * 100) // capacity


def bw_jitter(samples: list[int], window_size: int) -> int:
    """Compute average jitter (variation between consecutive samples)."""
    if window_size < 2:
        return 0
    total_diff: int = 0
    i: int = 1
    while i < window_size:
        prev: int = samples[i - 1]
        curr: int = samples[i]
        diff: int = curr - prev
        if diff < 0:
            diff = 0 - diff
        total_diff = total_diff + diff
        i = i + 1
    return total_diff // (window_size - 1)


def bw_variance_approx(samples: list[int], window_size: int) -> int:
    """Approximate variance (mean of squared deviations from mean)."""
    avg: int = bw_moving_average(samples, window_size)
    total: int = 0
    i: int = 0
    while i < window_size:
        s: int = samples[i]
        diff: int = s - avg
        total = total + diff * diff
        i = i + 1
    return total // window_size


def test_module() -> int:
    """Test bandwidth measurement."""
    passed: int = 0
    win: int = 5
    samples: list[int] = bw_init_zeros(win)
    head: list[int] = [0]

    # Test 1: record samples and compute average
    bw_record_sample(samples, head, win, 100)
    bw_record_sample(samples, head, win, 200)
    bw_record_sample(samples, head, win, 150)
    bw_record_sample(samples, head, win, 300)
    bw_record_sample(samples, head, win, 250)
    avg: int = bw_moving_average(samples, win)
    if avg == 200:
        passed = passed + 1

    # Test 2: peak detection
    pk: int = bw_peak(samples, win)
    if pk == 300:
        passed = passed + 1

    # Test 3: minimum detection
    mn: int = bw_minimum(samples, win)
    if mn == 100:
        passed = passed + 1

    # Test 4: utilization percentage
    util: int = bw_utilization_pct(750, 1000)
    if util == 75:
        passed = passed + 1

    # Test 5: jitter calculation
    jit: int = bw_jitter(samples, win)
    if jit > 0:
        passed = passed + 1

    return passed
