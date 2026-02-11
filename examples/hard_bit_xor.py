"""XOR tricks: swap, find single element, find missing number."""


def xor_swap(a: int, b: int) -> list[int]:
    """Swap two values using XOR and return as list."""
    x: int = a
    y: int = b
    x = x ^ y
    y = x ^ y
    x = x ^ y
    result: list[int] = [x, y]
    return result


def find_single(arr: list[int]) -> int:
    """Find the element that appears exactly once (all others appear twice)."""
    result: int = 0
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        result = result ^ arr[idx]
        idx = idx + 1
    return result


def find_missing(n: int, arr: list[int]) -> int:
    """Find missing number from 0..n given array of n elements."""
    xor_all: int = 0
    i: int = 0
    while i <= n:
        xor_all = xor_all ^ i
        i = i + 1
    xor_arr: int = 0
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        xor_arr = xor_arr ^ arr[idx]
        idx = idx + 1
    return xor_all ^ xor_arr


def xor_range(low: int, high: int) -> int:
    """Compute XOR of all integers from low to high inclusive."""
    result: int = 0
    val: int = low
    while val <= high:
        result = result ^ val
        val = val + 1
    return result


def test_module() -> int:
    passed: int = 0

    swapped: list[int] = xor_swap(3, 7)
    if swapped[0] == 7:
        passed = passed + 1
    if swapped[1] == 3:
        passed = passed + 1
    if find_single([1, 2, 3, 2, 1]) == 3:
        passed = passed + 1
    if find_single([5, 5, 9]) == 9:
        passed = passed + 1
    if find_missing(4, [0, 1, 3, 4]) == 2:
        passed = passed + 1
    if find_missing(3, [0, 1, 2]) == 3:
        passed = passed + 1
    if xor_range(1, 1) == 1:
        passed = passed + 1
    if xor_range(1, 3) == 0:
        passed = passed + 1

    return passed
