"""Frequency counting, histogram building, and mode finding with dicts."""


def frequency_count(arr: list[int]) -> dict[int, int]:
    """Count frequency of each element."""
    freq: dict[int, int] = {}
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        if val in freq:
            freq[val] = freq[val] + 1
        else:
            freq[val] = 1
        i = i + 1
    return freq


def most_frequent(arr: list[int]) -> int:
    """Find the most frequent element. Returns first one found on ties."""
    freq: dict[int, int] = frequency_count(arr)
    best_val: int = 0
    best_count: int = 0
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        c: int = freq[val]
        if c > best_count:
            best_count = c
            best_val = val
        i = i + 1
    return best_val


def least_frequent(arr: list[int]) -> int:
    """Find the least frequent element."""
    freq: dict[int, int] = frequency_count(arr)
    best_val: int = 0
    best_count: int = len(arr) + 1
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        c: int = freq[val]
        if c < best_count:
            best_count = c
            best_val = val
        i = i + 1
    return best_val


def elements_with_count(arr: list[int], target_count: int) -> list[int]:
    """Return elements that appear exactly target_count times."""
    freq: dict[int, int] = frequency_count(arr)
    result: list[int] = []
    seen: dict[int, int] = {}
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        if val not in seen:
            c: int = freq[val]
            if c == target_count:
                result.append(val)
            seen[val] = 1
        i = i + 1
    return result


def histogram_buckets(arr: list[int], bucket_size: int) -> dict[int, int]:
    """Build histogram with given bucket size."""
    hist: dict[int, int] = {}
    i: int = 0
    while i < len(arr):
        bucket_idx: int = arr[i] // bucket_size
        if bucket_idx in hist:
            hist[bucket_idx] = hist[bucket_idx] + 1
        else:
            hist[bucket_idx] = 1
        i = i + 1
    return hist


def unique_count(arr: list[int]) -> int:
    """Count number of unique elements."""
    seen: dict[int, int] = {}
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        seen[val] = 1
        i = i + 1
    count: int = 0
    for sk in seen:
        count = count + 1
    return count


def has_duplicates(arr: list[int]) -> int:
    """Return 1 if array has duplicate elements."""
    seen: dict[int, int] = {}
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        if val in seen:
            return 1
        seen[val] = 1
        i = i + 1
    return 0


def first_unique(arr: list[int]) -> int:
    """Find first element that appears exactly once. Returns -1 if none."""
    freq: dict[int, int] = frequency_count(arr)
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        c: int = freq[val]
        if c == 1:
            return val
        i = i + 1
    return -1


def test_module() -> int:
    """Test all frequency counting functions."""
    passed: int = 0
    freq: dict[int, int] = frequency_count([1, 2, 2, 3, 3, 3])
    if freq[1] == 1:
        passed = passed + 1
    if freq[3] == 3:
        passed = passed + 1
    mf: int = most_frequent([1, 2, 2, 3, 3, 3])
    if mf == 3:
        passed = passed + 1
    lf: int = least_frequent([1, 2, 2, 3, 3, 3])
    if lf == 1:
        passed = passed + 1
    ec: list[int] = elements_with_count([1, 2, 2, 3, 3, 3], 2)
    if len(ec) == 1:
        passed = passed + 1
    if ec[0] == 2:
        passed = passed + 1
    if unique_count([1, 1, 2, 3, 3]) == 3:
        passed = passed + 1
    if has_duplicates([1, 2, 3]) == 0:
        passed = passed + 1
    if has_duplicates([1, 2, 2]) == 1:
        passed = passed + 1
    fu: int = first_unique([2, 1, 2, 3, 3])
    if fu == 1:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
