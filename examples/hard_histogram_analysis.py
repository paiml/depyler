"""Histogram analysis: build histograms, find modes, and compute statistics."""


def build_histogram(values: list[int], num_bins: int, min_val: int, max_val: int) -> list[int]:
    """Build a histogram with given number of bins over a range."""
    bins: list[int] = []
    i: int = 0
    while i < num_bins:
        bins.append(0)
        i = i + 1
    range_size: int = max_val - min_val
    if range_size <= 0:
        return bins
    j: int = 0
    while j < len(values):
        v: int = values[j]
        if v >= min_val and v < max_val:
            bin_idx: int = (v - min_val) * num_bins // range_size
            if bin_idx >= num_bins:
                bin_idx = num_bins - 1
            bins[bin_idx] = bins[bin_idx] + 1
        elif v == max_val:
            last_bin: int = num_bins - 1
            bins[last_bin] = bins[last_bin] + 1
        j = j + 1
    return bins


def find_mode_index(histogram: list[int]) -> int:
    """Find the index of the bin with the highest count."""
    if len(histogram) == 0:
        return -1
    max_val: int = histogram[0]
    max_idx: int = 0
    i: int = 1
    while i < len(histogram):
        if histogram[i] > max_val:
            max_val = histogram[i]
            max_idx = i
        i = i + 1
    return max_idx


def histogram_entropy_approx(histogram: list[int]) -> int:
    """Approximate entropy * 1000 (integer math) of a histogram.
    Uses p * log2_approx(p) summed over bins."""
    total: int = 0
    i: int = 0
    while i < len(histogram):
        total = total + histogram[i]
        i = i + 1
    if total == 0:
        return 0
    entropy_1000: int = 0
    j: int = 0
    while j < len(histogram):
        count: int = histogram[j]
        if count > 0:
            p_1000: int = count * 1000 // total
            if p_1000 > 0:
                log2_approx: int = 0
                temp: int = p_1000
                while temp > 1:
                    log2_approx = log2_approx + 1
                    temp = temp // 2
                entropy_1000 = entropy_1000 + p_1000 * log2_approx
        j = j + 1
    result: int = entropy_1000 // 1000
    return result


def cumulative_histogram(histogram: list[int]) -> list[int]:
    """Convert a histogram to a cumulative histogram."""
    result: list[int] = []
    running: int = 0
    i: int = 0
    while i < len(histogram):
        running = running + histogram[i]
        result.append(running)
        i = i + 1
    return result


def test_module() -> int:
    """Test histogram analysis functions."""
    ok: int = 0

    vals: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    hist: list[int] = build_histogram(vals, 5, 1, 11)
    if len(hist) == 5 and hist[0] == 2 and hist[1] == 2:
        ok = ok + 1

    if find_mode_index(hist) >= 0:
        ok = ok + 1

    simple_hist: list[int] = [1, 5, 2]
    if find_mode_index(simple_hist) == 1:
        ok = ok + 1

    cum: list[int] = cumulative_histogram(simple_hist)
    if cum[0] == 1 and cum[1] == 6 and cum[2] == 8:
        ok = ok + 1

    empty_hist: list[int] = []
    if find_mode_index(empty_hist) == -1:
        ok = ok + 1

    if histogram_entropy_approx(simple_hist) >= 0:
        ok = ok + 1

    return ok
