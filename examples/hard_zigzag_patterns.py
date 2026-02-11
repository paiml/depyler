"""Zigzag traversal and encoding patterns.

Tests: alternating direction iteration, index arithmetic.
"""


def zigzag_order(arr: list[int], num_rows: int) -> list[int]:
    """Arrange elements in zigzag pattern across rows."""
    if num_rows <= 1 or len(arr) == 0:
        result: list[int] = []
        for v in arr:
            result.append(v)
        return result
    rows: list[list[int]] = []
    r: int = 0
    while r < num_rows:
        rows.append([])
        r += 1
    current_row: int = 0
    going_down: int = 1
    for val in arr:
        rows[current_row].append(val)
        if current_row == 0:
            going_down = 1
        if current_row == num_rows - 1:
            going_down = 0
        if going_down == 1:
            current_row += 1
        else:
            current_row -= 1
    result2: list[int] = []
    for row in rows:
        for v in row:
            result2.append(v)
    return result2


def zigzag_encode(n: int) -> int:
    """Zigzag encode a signed integer to unsigned."""
    if n >= 0:
        return 2 * n
    return 2 * (-n) - 1


def zigzag_decode(n: int) -> int:
    """Zigzag decode unsigned to signed."""
    if n % 2 == 0:
        return n // 2
    return -(n // 2 + 1)


def alternating_sum(arr: list[int]) -> int:
    """Compute alternating sum: a[0] - a[1] + a[2] - ..."""
    total: int = 0
    i: int = 0
    while i < len(arr):
        if i % 2 == 0:
            total += arr[i]
        else:
            total -= arr[i]
        i += 1
    return total


def reverse_alternating(arr: list[int]) -> list[int]:
    """Reverse elements at even indices, keep odd indices."""
    evens: list[int] = []
    i: int = 0
    while i < len(arr):
        evens.append(arr[i])
        i += 2
    result: list[int] = []
    ei: int = len(evens) - 1
    i = 0
    while i < len(arr):
        if i % 2 == 0:
            result.append(evens[ei])
            ei -= 1
        else:
            result.append(arr[i])
        i += 1
    return result


def test_module() -> int:
    """Test zigzag operations."""
    ok: int = 0

    z: list[int] = zigzag_order([1, 2, 3, 4, 5, 6], 3)
    if z == [1, 5, 2, 4, 6, 3]:
        ok += 1

    e: int = zigzag_encode(0)
    if e == 0:
        ok += 1
    e2: int = zigzag_encode(-1)
    if e2 == 1:
        ok += 1
    e3: int = zigzag_encode(1)
    if e3 == 2:
        ok += 1

    d: int = zigzag_decode(2)
    if d == 1:
        ok += 1

    asum: int = alternating_sum([1, 2, 3, 4])
    if asum == -2:
        ok += 1

    return ok
