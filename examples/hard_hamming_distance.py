"""Hamming distance operations.

Implements Hamming distance calculations for integers
and integer arrays, including error detection.
"""


def hamming_distance_int(a: int, b: int) -> int:
    """Compute Hamming distance between two integers (count differing bits)."""
    diff: int = a ^ b
    count: int = 0
    while diff > 0:
        count = count + (diff & 1)
        diff = diff >> 1
    return count


def hamming_distance_arrays(arr1: list[int], arr2: list[int], size: int) -> int:
    """Compute total Hamming distance between corresponding elements."""
    total: int = 0
    i: int = 0
    while i < size:
        d: int = hamming_distance_int(arr1[i], arr2[i])
        total = total + d
        i = i + 1
    return total


def hamming_weight(value: int) -> int:
    """Compute Hamming weight (number of set bits) of value."""
    count: int = 0
    v: int = value
    while v > 0:
        count = count + (v & 1)
        v = v >> 1
    return count


def min_hamming_in_set(values: list[int], size: int) -> int:
    """Find minimum Hamming distance between any pair in the set."""
    if size < 2:
        return 0
    min_dist: int = 64
    i: int = 0
    while i < size:
        j: int = i + 1
        while j < size:
            d: int = hamming_distance_int(values[i], values[j])
            if d < min_dist:
                min_dist = d
            j = j + 1
        i = i + 1
    return min_dist


def can_detect_errors(min_distance: int) -> int:
    """Return number of bit errors detectable given minimum Hamming distance."""
    result: int = min_distance - 1
    return result


def can_correct_errors(min_distance: int) -> int:
    """Return number of bit errors correctable given minimum Hamming distance."""
    result: int = (min_distance - 1) // 2
    return result


def test_module() -> int:
    """Test Hamming distance operations."""
    ok: int = 0

    d1: int = hamming_distance_int(0b1010, 0b0110)
    if d1 == 2:
        ok = ok + 1

    hw: int = hamming_weight(0b11011)
    if hw == 4:
        ok = ok + 1

    arr1: list[int] = [1, 2, 3]
    arr2: list[int] = [3, 2, 1]
    ad: int = hamming_distance_arrays(arr1, arr2, 3)
    if ad == 4:
        ok = ok + 1

    vals: list[int] = [0b000, 0b111, 0b110]
    md: int = min_hamming_in_set(vals, 3)
    if md == 1:
        ok = ok + 1

    detect: int = can_detect_errors(3)
    correct: int = can_correct_errors(3)
    if detect == 2 and correct == 1:
        ok = ok + 1

    return ok
