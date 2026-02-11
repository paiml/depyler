# Euclidean, Manhattan, Chebyshev, Hamming distance


def abs_val(x: int) -> int:
    if x < 0:
        return -x
    return x


def isqrt(n: int) -> int:
    if n <= 0:
        return 0
    x: int = n
    y: int = (x + 1) // 2
    while y < x:
        x = y
        y = (x + n // x) // 2
    return x


def euclidean_distance_sq(a: list[int], b: list[int]) -> int:
    # Returns squared Euclidean distance (to avoid sqrt)
    total: int = 0
    i: int = 0
    while i < len(a):
        diff: int = a[i] - b[i]
        total = total + diff * diff
        i = i + 1
    return total


def euclidean_distance(a: list[int], b: list[int], scale: int) -> int:
    # Returns distance * scale
    sq: int = euclidean_distance_sq(a, b)
    return isqrt(sq * scale * scale)


def manhattan_distance(a: list[int], b: list[int]) -> int:
    total: int = 0
    i: int = 0
    while i < len(a):
        total = total + abs_val(a[i] - b[i])
        i = i + 1
    return total


def chebyshev_distance(a: list[int], b: list[int]) -> int:
    max_diff: int = 0
    i: int = 0
    while i < len(a):
        diff: int = abs_val(a[i] - b[i])
        if diff > max_diff:
            max_diff = diff
        i = i + 1
    return max_diff


def hamming_distance(a: list[int], b: list[int]) -> int:
    count: int = 0
    i: int = 0
    while i < len(a):
        if a[i] != b[i]:
            count = count + 1
        i = i + 1
    return count


def minkowski_distance_p(a: list[int], b: list[int], p: int) -> int:
    # Returns sum of |a_i - b_i|^p (without the p-th root)
    total: int = 0
    i: int = 0
    while i < len(a):
        diff: int = abs_val(a[i] - b[i])
        power: int = 1
        j: int = 0
        while j < p:
            power = power * diff
            j = j + 1
        total = total + power
        i = i + 1
    return total


def test_module() -> int:
    passed: int = 0

    # Test 1: Manhattan distance
    a: list[int] = [0, 0]
    b: list[int] = [3, 4]
    if manhattan_distance(a, b) == 7:
        passed = passed + 1

    # Test 2: Euclidean distance squared (3-4-5 triangle)
    if euclidean_distance_sq(a, b) == 25:
        passed = passed + 1

    # Test 3: Euclidean distance scaled
    scale: int = 100
    d: int = euclidean_distance(a, b, scale)
    if abs_val(d - 500) < 5:
        passed = passed + 1

    # Test 4: Chebyshev distance
    if chebyshev_distance(a, b) == 4:
        passed = passed + 1

    # Test 5: Hamming distance
    c: list[int] = [1, 0, 1, 1, 0]
    e: list[int] = [1, 1, 0, 1, 0]
    if hamming_distance(c, e) == 2:
        passed = passed + 1

    # Test 6: zero distance
    if manhattan_distance(a, a) == 0 and euclidean_distance_sq(a, a) == 0:
        passed = passed + 1

    # Test 7: Minkowski p=1 = Manhattan
    if minkowski_distance_p(a, b, 1) == manhattan_distance(a, b):
        passed = passed + 1

    # Test 8: Minkowski p=2 = Euclidean squared
    if minkowski_distance_p(a, b, 2) == euclidean_distance_sq(a, b):
        passed = passed + 1

    return passed
