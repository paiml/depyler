"""Frequency counting in arrays: mode, frequency table, and top-k elements."""


def frequency_table(arr: list[int], unique_vals: list[int], counts: list[int]) -> int:
    """Build a frequency table. Populates unique_vals and counts lists.
    Returns number of unique values."""
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        found: int = 0
        j: int = 0
        while j < len(unique_vals):
            if unique_vals[j] == val:
                counts[j] = counts[j] + 1
                found = 1
                j = len(unique_vals)
            j = j + 1
        if found == 0:
            unique_vals.append(val)
            counts.append(1)
        i = i + 1
    return len(unique_vals)


def find_mode(arr: list[int]) -> int:
    """Find the most frequent element. Returns first one in case of tie."""
    if len(arr) == 0:
        return 0
    unique_vals: list[int] = []
    freqs: list[int] = []
    frequency_table(arr, unique_vals, freqs)
    max_freq: int = 0
    mode_val: int = arr[0]
    i: int = 0
    while i < len(unique_vals):
        if freqs[i] > max_freq:
            max_freq = freqs[i]
            mode_val = unique_vals[i]
        i = i + 1
    return mode_val


def count_elements_with_frequency(arr: list[int], target_freq: int) -> int:
    """Count how many distinct elements appear exactly target_freq times."""
    unique_vals: list[int] = []
    freqs: list[int] = []
    frequency_table(arr, unique_vals, freqs)
    count: int = 0
    i: int = 0
    while i < len(freqs):
        if freqs[i] == target_freq:
            count = count + 1
        i = i + 1
    return count


def majority_element_bm(arr: list[int]) -> int:
    """Boyer-Moore majority vote algorithm.
    Returns the candidate (verify separately if needed)."""
    if len(arr) == 0:
        return -1
    candidate: int = arr[0]
    count: int = 1
    i: int = 1
    while i < len(arr):
        if count == 0:
            candidate = arr[i]
            count = 1
        elif arr[i] == candidate:
            count = count + 1
        else:
            count = count - 1
        i = i + 1
    return candidate


def test_module() -> int:
    """Test frequency counting functions."""
    ok: int = 0

    arr1: list[int] = [1, 2, 2, 3, 3, 3]
    if find_mode(arr1) == 3:
        ok = ok + 1

    if count_elements_with_frequency(arr1, 2) == 1:
        ok = ok + 1

    if count_elements_with_frequency(arr1, 1) == 1:
        ok = ok + 1

    arr2: list[int] = [5, 5, 5, 3, 3]
    if majority_element_bm(arr2) == 5:
        ok = ok + 1

    uvals: list[int] = []
    ucnts: list[int] = []
    n: int = frequency_table(arr1, uvals, ucnts)
    if n == 3:
        ok = ok + 1

    empty: list[int] = []
    if majority_element_bm(empty) == -1:
        ok = ok + 1

    return ok
