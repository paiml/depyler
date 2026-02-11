"""Wave pattern generation: create wave arrays and detect wave patterns."""


def generate_wave(amplitude: int, period: int, length: int) -> list[int]:
    """Generate a triangular wave pattern as integer array.
    Goes up to amplitude, then down to -amplitude, repeating."""
    result: list[int] = []
    if period <= 0 or amplitude <= 0:
        i: int = 0
        while i < length:
            result.append(0)
            i = i + 1
        return result
    i2: int = 0
    while i2 < length:
        pos: int = i2 % period
        quarter: int = period // 4
        if quarter == 0:
            quarter = 1
        half: int = period // 2
        if pos <= quarter:
            val: int = amplitude * pos // quarter
        elif pos <= half:
            val2: int = amplitude * (half - pos) // quarter
            val = val2
        elif pos <= half + quarter:
            val3: int = -amplitude * (pos - half) // quarter
            val = val3
        else:
            val4: int = -amplitude * (period - pos) // quarter
            val = val4
        result.append(val)
        i2 = i2 + 1
    return result


def is_wave_sorted(arr: list[int]) -> int:
    """Check if array is wave-sorted: a[0] >= a[1] <= a[2] >= a[3] <= ...
    Returns 1 if wave-sorted, 0 otherwise."""
    n: int = len(arr)
    if n <= 1:
        return 1
    i: int = 0
    while i < n - 1:
        next_idx: int = i + 1
        if i % 2 == 0:
            if arr[i] < arr[next_idx]:
                return 0
        else:
            if arr[i] > arr[next_idx]:
                return 0
        i = i + 1
    return 1


def make_wave_sorted(arr: list[int]) -> list[int]:
    """Convert array to wave-sorted form by swapping adjacent elements."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    j: int = 0
    n: int = len(result)
    while j < n - 1:
        next_idx: int = j + 1
        if j % 2 == 0:
            if result[j] < result[next_idx]:
                tmp: int = result[j]
                result[j] = result[next_idx]
                result[next_idx] = tmp
        else:
            if result[j] > result[next_idx]:
                tmp2: int = result[j]
                result[j] = result[next_idx]
                result[next_idx] = tmp2
        j = j + 1
    return result


def count_peaks_and_valleys(arr: list[int]) -> list[int]:
    """Count peaks and valleys in array. Returns [peaks, valleys]."""
    peaks: int = 0
    valleys: int = 0
    n: int = len(arr)
    if n < 3:
        result: list[int] = [0, 0]
        return result
    i: int = 1
    last_idx: int = n - 1
    while i < last_idx:
        prev: int = i - 1
        next_i: int = i + 1
        if arr[i] > arr[prev] and arr[i] > arr[next_i]:
            peaks = peaks + 1
        elif arr[i] < arr[prev] and arr[i] < arr[next_i]:
            valleys = valleys + 1
        i = i + 1
    result2: list[int] = [peaks, valleys]
    return result2


def test_module() -> int:
    """Test wave pattern functions."""
    ok: int = 0

    wave: list[int] = generate_wave(10, 8, 8)
    if len(wave) == 8:
        ok = ok + 1

    arr1: list[int] = [5, 1, 4, 2, 3]
    if is_wave_sorted(arr1) == 1:
        ok = ok + 1

    arr2: list[int] = [1, 2, 3, 4, 5]
    if is_wave_sorted(arr2) == 0:
        ok = ok + 1

    ws: list[int] = make_wave_sorted(arr2)
    if is_wave_sorted(ws) == 1:
        ok = ok + 1

    arr3: list[int] = [1, 3, 1, 3, 1]
    pv: list[int] = count_peaks_and_valleys(arr3)
    if pv[0] == 2 and pv[1] == 1:
        ok = ok + 1

    short: list[int] = [1, 2]
    pv2: list[int] = count_peaks_and_valleys(short)
    if pv2[0] == 0 and pv2[1] == 0:
        ok = ok + 1

    return ok
