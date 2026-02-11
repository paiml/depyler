"""Array rearrangement patterns.

Tests: even-odd split, positive-negative, Dutch flag partition, segregate.
"""


def even_odd_split(arr: list[int]) -> list[int]:
    """Rearrange so even numbers come before odd numbers."""
    evens: list[int] = []
    odds: list[int] = []
    i: int = 0
    while i < len(arr):
        if arr[i] % 2 == 0:
            evens.append(arr[i])
        else:
            odds.append(arr[i])
        i = i + 1
    result: list[int] = []
    i = 0
    while i < len(evens):
        result.append(evens[i])
        i = i + 1
    i = 0
    while i < len(odds):
        result.append(odds[i])
        i = i + 1
    return result


def positive_negative_split(arr: list[int]) -> list[int]:
    """Rearrange so negatives come before positives, zeroes in between."""
    neg: list[int] = []
    zero: list[int] = []
    pos: list[int] = []
    i: int = 0
    while i < len(arr):
        if arr[i] < 0:
            neg.append(arr[i])
        elif arr[i] == 0:
            zero.append(arr[i])
        else:
            pos.append(arr[i])
        i = i + 1
    result: list[int] = []
    i = 0
    while i < len(neg):
        result.append(neg[i])
        i = i + 1
    i = 0
    while i < len(zero):
        result.append(zero[i])
        i = i + 1
    i = 0
    while i < len(pos):
        result.append(pos[i])
        i = i + 1
    return result


def dutch_flag_partition(arr: list[int], pivot: int) -> list[int]:
    """Three-way partition: < pivot, == pivot, > pivot."""
    lo: list[int] = []
    eq: list[int] = []
    hi: list[int] = []
    i: int = 0
    while i < len(arr):
        if arr[i] < pivot:
            lo.append(arr[i])
        elif arr[i] == pivot:
            eq.append(arr[i])
        else:
            hi.append(arr[i])
        i = i + 1
    result: list[int] = []
    i = 0
    while i < len(lo):
        result.append(lo[i])
        i = i + 1
    i = 0
    while i < len(eq):
        result.append(eq[i])
        i = i + 1
    i = 0
    while i < len(hi):
        result.append(hi[i])
        i = i + 1
    return result


def count_arrangement_inversions(arr: list[int]) -> int:
    """Count number of positions where arr[i] > arr[i+1]."""
    count: int = 0
    i: int = 0
    while i < len(arr) - 1:
        if arr[i] > arr[i + 1]:
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test array rearrangement operations."""
    ok: int = 0
    eo: list[int] = even_odd_split([3, 2, 7, 4, 6, 1])
    if eo[0] == 2 and eo[1] == 4:
        ok = ok + 1
    pn: list[int] = positive_negative_split([-1, 3, 0, -2, 5])
    if pn[0] == -1 and pn[1] == -2:
        ok = ok + 1
    df: list[int] = dutch_flag_partition([3, 1, 4, 1, 5, 9, 2, 6], 4)
    if df[0] < 4 and df[len(df) - 1] > 4:
        ok = ok + 1
    if count_arrangement_inversions([1, 3, 2, 4, 1]) == 2:
        ok = ok + 1
    if count_arrangement_inversions([1, 2, 3, 4, 5]) == 0:
        ok = ok + 1
    return ok
