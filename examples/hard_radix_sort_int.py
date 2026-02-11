"""Radix sort for non-negative integers."""


def get_max_val(arr: list[int]) -> int:
    """Find maximum value in array."""
    if len(arr) == 0:
        return 0
    mx: int = arr[0]
    i: int = 1
    while i < len(arr):
        if arr[i] > mx:
            mx = arr[i]
        i = i + 1
    return mx


def counting_sort_digit(arr: list[int], exp: int) -> list[int]:
    """Sort array by digit at position exp using counting sort."""
    n: int = len(arr)
    output: list[int] = []
    count: list[int] = []
    i: int = 0
    while i < n:
        output.append(0)
        i = i + 1
    i = 0
    while i < 10:
        count.append(0)
        i = i + 1
    i = 0
    while i < n:
        digit: int = (arr[i] // exp) % 10
        count[digit] = count[digit] + 1
        i = i + 1
    i = 1
    while i < 10:
        count[i] = count[i] + count[i - 1]
        i = i + 1
    i = n - 1
    while i >= 0:
        digit2: int = (arr[i] // exp) % 10
        count[digit2] = count[digit2] - 1
        output[count[digit2]] = arr[i]
        i = i - 1
    return output


def radix_sort(arr: list[int]) -> list[int]:
    """Sort non-negative integers using radix sort."""
    if len(arr) <= 1:
        return arr
    mx: int = get_max_val(arr)
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i])
        i = i + 1
    exp: int = 1
    while mx // exp > 0:
        result = counting_sort_digit(result, exp)
        exp = exp * 10
    return result


def is_sorted_asc(arr: list[int]) -> int:
    """Returns 1 if sorted ascending."""
    i: int = 1
    while i < len(arr):
        if arr[i] < arr[i - 1]:
            return 0
        i = i + 1
    return 1


def test_module() -> int:
    """Test radix sort."""
    ok: int = 0
    a1: list[int] = [170, 45, 75, 90, 802, 24, 2, 66]
    r1: list[int] = radix_sort(a1)
    if is_sorted_asc(r1) == 1:
        ok = ok + 1
    if r1[0] == 2:
        ok = ok + 1
    if r1[7] == 802:
        ok = ok + 1
    a2: list[int] = [1]
    r2: list[int] = radix_sort(a2)
    if len(r2) == 1:
        ok = ok + 1
    a3: list[int] = [3, 1, 2]
    r3: list[int] = radix_sort(a3)
    if r3[0] == 1 and r3[2] == 3:
        ok = ok + 1
    empty: list[int] = []
    r4: list[int] = radix_sort(empty)
    if len(r4) == 0:
        ok = ok + 1
    a5: list[int] = [100, 10, 1]
    r5: list[int] = radix_sort(a5)
    if is_sorted_asc(r5) == 1:
        ok = ok + 1
    return ok
