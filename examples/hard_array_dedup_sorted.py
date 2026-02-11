"""Remove duplicates from a sorted array in-place style."""


def dedup_sorted(arr: list[int]) -> list[int]:
    """Remove duplicates from sorted array, return new list."""
    if len(arr) == 0:
        return []
    result: list[int] = []
    result.append(arr[0])
    i: int = 1
    while i < len(arr):
        if arr[i] != arr[i - 1]:
            result.append(arr[i])
        i = i + 1
    return result


def count_unique_sorted(arr: list[int]) -> int:
    """Count unique elements in sorted array."""
    if len(arr) == 0:
        return 0
    count: int = 1
    i: int = 1
    while i < len(arr):
        if arr[i] != arr[i - 1]:
            count = count + 1
        i = i + 1
    return count


def dedup_keep_last(arr: list[int]) -> list[int]:
    """Remove duplicates keeping last occurrence from sorted array."""
    if len(arr) == 0:
        return []
    result: list[int] = []
    i: int = 0
    while i < len(arr) - 1:
        if arr[i] != arr[i + 1]:
            result.append(arr[i])
        i = i + 1
    result.append(arr[len(arr) - 1])
    return result


def has_duplicates_sorted(arr: list[int]) -> int:
    """Returns 1 if sorted array has duplicates."""
    i: int = 1
    while i < len(arr):
        if arr[i] == arr[i - 1]:
            return 1
        i = i + 1
    return 0


def test_module() -> int:
    """Test dedup operations."""
    ok: int = 0
    a1: list[int] = [1, 1, 2, 3, 3, 3, 4]
    r1: list[int] = dedup_sorted(a1)
    if len(r1) == 4:
        ok = ok + 1
    if r1[0] == 1:
        ok = ok + 1
    if r1[3] == 4:
        ok = ok + 1
    if count_unique_sorted(a1) == 4:
        ok = ok + 1
    r2: list[int] = dedup_keep_last(a1)
    if len(r2) == 4:
        ok = ok + 1
    if has_duplicates_sorted(a1) == 1:
        ok = ok + 1
    a2: list[int] = [1, 2, 3]
    if has_duplicates_sorted(a2) == 0:
        ok = ok + 1
    return ok
