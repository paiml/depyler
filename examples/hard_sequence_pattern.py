"""Pattern detection in sequences.

Implements algorithms to detect repeating patterns,
monotonic subsequences, and periodic elements.
"""


def detect_period(arr: list[int], size: int) -> int:
    """Detect the shortest repeating period in array. Returns period length or size if none."""
    period: int = 1
    while period <= size // 2:
        is_periodic: int = 1
        i: int = 0
        while i < size:
            if arr[i] != arr[i % period]:
                is_periodic = 0
                i = size
            i = i + 1
        if is_periodic == 1:
            return period
        period = period + 1
    return size


def longest_increasing_run(arr: list[int], size: int) -> int:
    """Find length of the longest strictly increasing run."""
    if size == 0:
        return 0
    max_run: int = 1
    current_run: int = 1
    i: int = 1
    while i < size:
        prev_idx: int = i - 1
        if arr[i] > arr[prev_idx]:
            current_run = current_run + 1
            if current_run > max_run:
                max_run = current_run
        else:
            current_run = 1
        i = i + 1
    return max_run


def count_plateaus(arr: list[int], size: int) -> int:
    """Count plateaus: consecutive equal elements of length >= 2."""
    if size < 2:
        return 0
    plateaus: int = 0
    i: int = 0
    while i < size:
        run_len: int = 1
        while i + run_len < size and arr[i + run_len] == arr[i]:
            run_len = run_len + 1
        if run_len >= 2:
            plateaus = plateaus + 1
        i = i + run_len
    return plateaus


def is_zigzag(arr: list[int], size: int) -> int:
    """Check if array is a zigzag pattern (alternating up/down). Returns 1 if yes."""
    if size <= 2:
        return 1
    i: int = 1
    limit: int = size - 1
    while i < limit:
        prev_idx: int = i - 1
        next_idx: int = i + 1
        above: int = 0
        below: int = 0
        if arr[i] > arr[prev_idx] and arr[i] > arr[next_idx]:
            above = 1
        if arr[i] < arr[prev_idx] and arr[i] < arr[next_idx]:
            below = 1
        if above == 0 and below == 0:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test sequence pattern operations."""
    ok: int = 0

    arr1: list[int] = [1, 2, 3, 1, 2, 3]
    period: int = detect_period(arr1, 6)
    if period == 3:
        ok = ok + 1

    arr2: list[int] = [1, 3, 5, 2, 8, 9, 10]
    run: int = longest_increasing_run(arr2, 7)
    if run == 3:
        ok = ok + 1

    arr3: list[int] = [1, 1, 2, 3, 3, 3, 4]
    plat: int = count_plateaus(arr3, 7)
    if plat == 2:
        ok = ok + 1

    arr4: list[int] = [1, 3, 2, 4, 3]
    zz: int = is_zigzag(arr4, 5)
    if zz == 1:
        ok = ok + 1

    return ok
