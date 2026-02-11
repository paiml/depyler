# Pathological dict: Accumulation patterns (counting, grouping, bucketing)
# Tests: dict[str, int] and dict[int, int] for frequency/grouping tasks


def frequency_count(items: list[int]) -> dict[int, int]:
    """Count frequency of each integer."""
    freq: dict[int, int] = {}
    i: int = 0
    while i < len(items):
        val: int = items[i]
        if val in freq:
            freq[val] = freq[val] + 1
        else:
            freq[val] = 1
        i = i + 1
    return freq


def bucket_by_range(vals: list[int], bucket_size: int) -> dict[int, int]:
    """Bucket values into ranges and count per bucket.
    Bucket id = val // bucket_size."""
    buckets: dict[int, int] = {}
    i: int = 0
    while i < len(vals):
        bid: int = vals[i] // bucket_size
        if bid in buckets:
            buckets[bid] = buckets[bid] + 1
        else:
            buckets[bid] = 1
        i = i + 1
    return buckets


def running_frequency(items: list[int]) -> list[int]:
    """For each position, return how many times that value has appeared so far."""
    freq: dict[int, int] = {}
    result: list[int] = []
    i: int = 0
    while i < len(items):
        val: int = items[i]
        if val in freq:
            freq[val] = freq[val] + 1
        else:
            freq[val] = 1
        result.append(freq[val])
        i = i + 1
    return result


def mode_value(items: list[int]) -> int:
    """Find the mode (most frequent value). First occurrence wins ties."""
    if len(items) == 0:
        return 0 - 1
    freq: dict[int, int] = frequency_count(items)
    best_val: int = items[0]
    best_count: int = 0
    i: int = 0
    while i < len(items):
        val: int = items[i]
        cnt: int = freq[val]
        if cnt > best_count:
            best_count = cnt
            best_val = val
        i = i + 1
    return best_val


def count_pairs_with_sum(items: list[int], target: int) -> int:
    """Count pairs (i,j) where i<j and items[i]+items[j]==target using dict."""
    seen: dict[int, int] = {}
    count: int = 0
    i: int = 0
    while i < len(items):
        complement: int = target - items[i]
        if complement in seen:
            count = count + seen[complement]
        if items[i] in seen:
            seen[items[i]] = seen[items[i]] + 1
        else:
            seen[items[i]] = 1
        i = i + 1
    return count


def test_module() -> int:
    passed: int = 0
    # Test 1: frequency count
    freq: dict[int, int] = frequency_count([1, 2, 2, 3, 3, 3])
    if freq[3] == 3:
        passed = passed + 1
    # Test 2: bucket
    buckets: dict[int, int] = bucket_by_range([1, 5, 12, 15, 23], 10)
    if buckets[0] == 2:
        passed = passed + 1
    # Test 3: running frequency
    rf: list[int] = running_frequency([1, 2, 1, 3, 2, 1])
    if rf[5] == 3:
        passed = passed + 1
    # Test 4: mode
    if mode_value([1, 2, 2, 3, 3, 3, 2, 2]) == 2:
        passed = passed + 1
    # Test 5: pairs with sum
    if count_pairs_with_sum([1, 2, 3, 4, 5], 5) == 2:
        passed = passed + 1
    # Test 6: empty mode
    if mode_value([]) == 0 - 1:
        passed = passed + 1
    # Test 7: single element freq
    f2: dict[int, int] = frequency_count([42])
    if f2[42] == 1:
        passed = passed + 1
    return passed
