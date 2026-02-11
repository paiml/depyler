"""Find all leaders in an array. A leader is greater than all elements to its right."""


def find_leaders(arr: list[int]) -> list[int]:
    """Find all leaders in array scanning from right."""
    length: int = len(arr)
    if length == 0:
        return []
    leaders: list[int] = []
    max_from_right: int = arr[length - 1]
    leaders.append(max_from_right)
    i: int = length - 2
    while i >= 0:
        if arr[i] > max_from_right:
            max_from_right = arr[i]
            leaders.append(arr[i])
        i = i - 1
    return leaders


def count_leaders(arr: list[int]) -> int:
    """Count number of leaders in array."""
    result: list[int] = find_leaders(arr)
    return len(result)


def is_leader(arr: list[int], idx: int) -> int:
    """Returns 1 if element at idx is a leader."""
    length: int = len(arr)
    if idx < 0:
        return 0
    if idx >= length:
        return 0
    val: int = arr[idx]
    j: int = idx + 1
    while j < length:
        if arr[j] >= val:
            return 0
        j = j + 1
    return 1


def max_leader(arr: list[int]) -> int:
    """Return maximum leader value."""
    leaders: list[int] = find_leaders(arr)
    if len(leaders) == 0:
        return 0
    best: int = leaders[0]
    i: int = 1
    while i < len(leaders):
        if leaders[i] > best:
            best = leaders[i]
        i = i + 1
    return best


def test_module() -> int:
    """Test leader operations."""
    ok: int = 0
    a1: list[int] = [16, 17, 4, 3, 5, 2]
    if count_leaders(a1) == 3:
        ok = ok + 1
    if is_leader(a1, 1) == 1:
        ok = ok + 1
    if is_leader(a1, 2) == 0:
        ok = ok + 1
    if is_leader(a1, 5) == 1:
        ok = ok + 1
    if max_leader(a1) == 17:
        ok = ok + 1
    a2: list[int] = [5, 4, 3, 2, 1]
    if count_leaders(a2) == 5:
        ok = ok + 1
    a3: list[int] = [1]
    if count_leaders(a3) == 1:
        ok = ok + 1
    return ok
