# Distance calculations (Manhattan, Euclidean approx, Hamming)
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def manhattan_distance(x1: int, y1: int, x2: int, y2: int) -> int:
    """Compute Manhattan distance between two points."""
    dx: int = x1 - x2
    dy: int = y1 - y2
    if dx < 0:
        dx = 0 - dx
    if dy < 0:
        dy = 0 - dy
    return dx + dy


def chebyshev_distance(x1: int, y1: int, x2: int, y2: int) -> int:
    """Compute Chebyshev distance (max of abs differences)."""
    dx: int = x1 - x2
    dy: int = y1 - y2
    if dx < 0:
        dx = 0 - dx
    if dy < 0:
        dy = 0 - dy
    if dx > dy:
        return dx
    return dy


def hamming_distance_int(a: int, b: int) -> int:
    """Compute Hamming distance between two integers (bit difference count)."""
    xor: int = a ^ b
    count: int = 0
    while xor > 0:
        count = count + (xor & 1)
        xor = xor >> 1
    return count


def list_distance_manhattan(a: list[int], b: list[int]) -> int:
    """Compute Manhattan distance between two equal-length int vectors."""
    total: int = 0
    i: int = 0
    limit: int = len(a)
    if len(b) < limit:
        limit = len(b)
    while i < limit:
        diff: int = a[i] - b[i]
        if diff < 0:
            diff = 0 - diff
        total = total + diff
        i = i + 1
    return total


def squared_euclidean(x1: int, y1: int, x2: int, y2: int) -> int:
    """Compute squared Euclidean distance (avoids sqrt)."""
    dx: int = x1 - x2
    dy: int = y1 - y2
    return dx * dx + dy * dy


def test_module() -> int:
    assert manhattan_distance(0, 0, 3, 4) == 7
    assert manhattan_distance(1, 1, 1, 1) == 0
    assert manhattan_distance(-1, -1, 2, 3) == 7
    assert chebyshev_distance(0, 0, 3, 4) == 4
    assert chebyshev_distance(1, 1, 1, 1) == 0
    assert chebyshev_distance(0, 0, 5, 3) == 5
    assert hamming_distance_int(7, 0) == 3
    assert hamming_distance_int(0, 0) == 0
    assert hamming_distance_int(15, 0) == 4
    assert list_distance_manhattan([1, 2, 3], [4, 5, 6]) == 9
    assert list_distance_manhattan([0, 0], [0, 0]) == 0
    assert squared_euclidean(0, 0, 3, 4) == 25
    assert squared_euclidean(1, 1, 1, 1) == 0
    return 0


if __name__ == "__main__":
    test_module()
