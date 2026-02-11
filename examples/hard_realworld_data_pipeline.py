"""Real-world multi-stage data transformation pipeline.

Mimics: ETL pipelines, pandas DataFrame operations, Apache Beam transforms.
Chain of filter, map, aggregate, normalize operations on tabular data.
"""


def pipeline_filter_positive(data: list[int]) -> list[int]:
    """Filter stage: keep only positive values."""
    result: list[int] = []
    idx: int = 0
    while idx < len(data):
        if data[idx] > 0:
            result.append(data[idx])
        idx = idx + 1
    return result


def pipeline_map_double(data: list[int]) -> list[int]:
    """Map stage: double each value."""
    result: list[int] = []
    idx: int = 0
    while idx < len(data):
        result.append(data[idx] * 2)
        idx = idx + 1
    return result


def pipeline_map_clamp(data: list[int], low: int, high: int) -> list[int]:
    """Map stage: clamp values to [low, high] range."""
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


def pipeline_deduplicate(data: list[int]) -> list[int]:
    """Dedup stage: remove consecutive duplicates."""
    if len(data) == 0:
        return []
    result: list[int] = [data[0]]
    idx: int = 1
    while idx < len(data):
        if data[idx] != data[idx - 1]:
            result.append(data[idx])
        idx = idx + 1
    return result


def pipeline_running_sum(data: list[int]) -> list[int]:
    """Aggregate stage: compute running cumulative sum."""
    result: list[int] = []
    total: int = 0
    idx: int = 0
    while idx < len(data):
        total = total + data[idx]
        result.append(total)
        idx = idx + 1
    return result


def pipeline_normalize_to_100(data: list[int]) -> list[int]:
    """Normalize stage: scale values so max becomes 100."""
    if len(data) == 0:
        return []
    max_val: int = data[0]
    idx: int = 1
    while idx < len(data):
        if data[idx] > max_val:
            max_val = data[idx]
        idx = idx + 1
    if max_val == 0:
        return data
    result: list[int] = []
    idx2: int = 0
    while idx2 < len(data):
        result.append((data[idx2] * 100) // max_val)
        idx2 = idx2 + 1
    return result


def pipeline_bucket_count(data: list[int], num_buckets: int, max_val: int) -> list[int]:
    """Histogram stage: count values in each bucket."""
    buckets: list[int] = []
    bi: int = 0
    while bi < num_buckets:
        buckets.append(0)
        bi = bi + 1
    bucket_size: int = max_val // num_buckets
    if bucket_size == 0:
        bucket_size = 1
    idx: int = 0
    while idx < len(data):
        bucket_idx: int = data[idx] // bucket_size
        if bucket_idx >= num_buckets:
            bucket_idx = num_buckets - 1
        if bucket_idx < 0:
            bucket_idx = 0
        buckets[bucket_idx] = buckets[bucket_idx] + 1
        idx = idx + 1
    return buckets


def pipeline_top_n(data: list[int], n: int) -> list[int]:
    """Select stage: return top N largest values (sorted descending)."""
    sorted_data: list[int] = []
    idx: int = 0
    while idx < len(data):
        sorted_data.append(data[idx])
        idx = idx + 1
    # Simple selection sort descending
    si: int = 0
    while si < len(sorted_data) and si < n:
        max_idx: int = si
        sj: int = si + 1
        while sj < len(sorted_data):
            if sorted_data[sj] > sorted_data[max_idx]:
                max_idx = sj
            sj = sj + 1
        tmp: int = sorted_data[si]
        sorted_data[si] = sorted_data[max_idx]
        sorted_data[max_idx] = tmp
        si = si + 1
    result: list[int] = []
    ri: int = 0
    while ri < n and ri < len(sorted_data):
        result.append(sorted_data[ri])
        ri = ri + 1
    return result


def run_full_pipeline(raw: list[int]) -> list[int]:
    """Execute full ETL pipeline: filter -> map -> clamp -> dedup -> normalize."""
    stage1: list[int] = pipeline_filter_positive(raw)
    stage2: list[int] = pipeline_map_double(stage1)
    stage3: list[int] = pipeline_map_clamp(stage2, 0, 200)
    stage4: list[int] = pipeline_deduplicate(stage3)
    stage5: list[int] = pipeline_normalize_to_100(stage4)
    return stage5


def pipeline_sum(data: list[int]) -> int:
    """Compute sum of all elements."""
    total: int = 0
    idx: int = 0
    while idx < len(data):
        total = total + data[idx]
        idx = idx + 1
    return total


def test_module() -> int:
    """Test data pipeline module."""
    passed: int = 0

    # Test 1: filter positive
    filtered: list[int] = pipeline_filter_positive([-3, 5, -1, 10, 0, 7])
    if len(filtered) == 3 and filtered[0] == 5:
        passed = passed + 1

    # Test 2: map double
    doubled: list[int] = pipeline_map_double([1, 2, 3])
    if doubled[0] == 2 and doubled[2] == 6:
        passed = passed + 1

    # Test 3: clamp
    clamped: list[int] = pipeline_map_clamp([1, 50, 200, 300], 10, 250)
    if clamped[0] == 10 and clamped[3] == 250:
        passed = passed + 1

    # Test 4: dedup
    deduped: list[int] = pipeline_deduplicate([1, 1, 2, 2, 2, 3, 1])
    if len(deduped) == 4:
        passed = passed + 1

    # Test 5: running sum
    rsum: list[int] = pipeline_running_sum([1, 2, 3, 4])
    if rsum[3] == 10:
        passed = passed + 1

    # Test 6: normalize
    normed: list[int] = pipeline_normalize_to_100([25, 50, 100])
    if normed[0] == 25 and normed[2] == 100:
        passed = passed + 1

    # Test 7: top N
    top: list[int] = pipeline_top_n([3, 1, 4, 1, 5, 9, 2, 6], 3)
    if top[0] == 9 and top[1] == 6 and top[2] == 5:
        passed = passed + 1

    # Test 8: full pipeline
    raw: list[int] = [-5, 10, -2, 20, 10, 30]
    result: list[int] = run_full_pipeline(raw)
    if len(result) > 0:
        passed = passed + 1

    return passed
