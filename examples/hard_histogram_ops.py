# Build histogram, percentile from histogram


def build_histogram(data: list[int], num_bins: int, min_val: int, max_val: int) -> list[int]:
    bins: list[int] = []
    i: int = 0
    while i < num_bins:
        bins.append(0)
        i = i + 1
    bin_width: int = (max_val - min_val) // num_bins
    if bin_width == 0:
        bin_width = 1
    j: int = 0
    while j < len(data):
        idx: int = (data[j] - min_val) // bin_width
        if idx < 0:
            idx = 0
        if idx >= num_bins:
            idx = num_bins - 1
        bins[idx] = bins[idx] + 1
        j = j + 1
    return bins


def histogram_sum(bins: list[int]) -> int:
    total: int = 0
    i: int = 0
    while i < len(bins):
        total = total + bins[i]
        i = i + 1
    return total


def histogram_max_bin(bins: list[int]) -> int:
    max_val: int = 0
    i: int = 0
    while i < len(bins):
        if bins[i] > max_val:
            max_val = bins[i]
        i = i + 1
    return max_val


def percentile_from_sorted(data: list[int], p: int) -> int:
    # p is percentile 0-100
    n: int = len(data)
    if n == 0:
        return 0
    idx: int = p * (n - 1) // 100
    return data[idx]


def histogram_mode_bin(bins: list[int]) -> int:
    # Returns index of the mode bin
    max_val: int = 0
    mode_idx: int = 0
    i: int = 0
    while i < len(bins):
        if bins[i] > max_val:
            max_val = bins[i]
            mode_idx = i
        i = i + 1
    return mode_idx


def sort_list(data: list[int]) -> list[int]:
    # Simple insertion sort
    result: list[int] = []
    i: int = 0
    while i < len(data):
        result.append(data[i])
        i = i + 1
    i = 1
    while i < len(result):
        key: int = result[i]
        j: int = i - 1
        while j >= 0 and result[j] > key:
            result[j + 1] = result[j]
            j = j - 1
        result[j + 1] = key
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0

    # Test 1: histogram sum equals data length
    data: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    bins: list[int] = build_histogram(data, 5, 1, 11)
    if histogram_sum(bins) == 10:
        passed = passed + 1

    # Test 2: uniform data -> equal bins
    if bins[0] == 2 and bins[1] == 2:
        passed = passed + 1

    # Test 3: max bin
    data2: list[int] = [1, 1, 1, 5, 5, 9]
    bins2: list[int] = build_histogram(data2, 3, 1, 10)
    if histogram_max_bin(bins2) == 3:
        passed = passed + 1

    # Test 4: mode bin
    if histogram_mode_bin(bins2) == 0:
        passed = passed + 1

    # Test 5: percentile 0 = min
    sorted_data: list[int] = sort_list(data)
    if percentile_from_sorted(sorted_data, 0) == 1:
        passed = passed + 1

    # Test 6: percentile 100 = max
    if percentile_from_sorted(sorted_data, 100) == 10:
        passed = passed + 1

    # Test 7: percentile 50 = median-ish
    med: int = percentile_from_sorted(sorted_data, 50)
    if med == 5 or med == 6:
        passed = passed + 1

    return passed
