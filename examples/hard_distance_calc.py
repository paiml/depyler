"""Distance calculations: Euclidean (integer approx), Hamming, and Minkowski."""


def squared_euclidean(x1: int, y1: int, x2: int, y2: int) -> int:
    """Calculate squared Euclidean distance (avoids sqrt)."""
    dx: int = x2 - x1
    dy: int = y2 - y1
    return dx * dx + dy * dy


def integer_sqrt(n: int) -> int:
    """Integer square root using Newton's method."""
    if n < 0:
        return 0
    if n == 0:
        return 0
    x: int = n
    y: int = (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x


def euclidean_distance_approx(x1: int, y1: int, x2: int, y2: int) -> int:
    """Approximate Euclidean distance as integer sqrt of squared distance."""
    sq: int = squared_euclidean(x1, y1, x2, y2)
    result: int = integer_sqrt(sq)
    return result


def hamming_distance_int(a: int, b: int) -> int:
    """Hamming distance between two integers (number of differing bits)."""
    xor_val: int = a ^ b
    if xor_val < 0:
        xor_val = -xor_val
    count: int = 0
    while xor_val > 0:
        count = count + (xor_val & 1)
        xor_val = xor_val >> 1
    return count


def taxicab_distance_nd(point_a: list[int], point_b: list[int]) -> int:
    """Calculate taxicab (Manhattan) distance in N dimensions."""
    n: int = len(point_a)
    if len(point_b) < n:
        n = len(point_b)
    total: int = 0
    i: int = 0
    while i < n:
        diff: int = point_a[i] - point_b[i]
        if diff < 0:
            diff = -diff
        total = total + diff
        i = i + 1
    return total


def test_module() -> int:
    """Test distance calculation functions."""
    ok: int = 0

    if squared_euclidean(0, 0, 3, 4) == 25:
        ok = ok + 1

    if integer_sqrt(25) == 5:
        ok = ok + 1

    if integer_sqrt(0) == 0:
        ok = ok + 1

    if euclidean_distance_approx(0, 0, 3, 4) == 5:
        ok = ok + 1

    if hamming_distance_int(7, 0) == 3:
        ok = ok + 1

    a: list[int] = [1, 2, 3]
    b: list[int] = [4, 6, 3]
    if taxicab_distance_nd(a, b) == 7:
        ok = ok + 1

    return ok
