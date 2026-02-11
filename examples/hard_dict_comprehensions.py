# Dict and list comprehension patterns for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def squares_dict(n: int) -> dict[int, int]:
    """Create a dict mapping i -> i*i for 1..n."""
    result: dict[int, int] = {}
    i: int = 1
    while i <= n:
        result[i] = i * i
        i = i + 1
    return result


def invert_dict(d: dict[int, int]) -> dict[int, int]:
    """Invert a dict swapping keys and values."""
    result: dict[int, int] = {}
    for k in d:
        result[d[k]] = k
    return result


def count_elements(nums: list[int]) -> dict[int, int]:
    """Count frequency of each element."""
    result: dict[int, int] = {}
    for n in nums:
        if n in result:
            result[n] = result[n] + 1
        else:
            result[n] = 1
    return result


def merge_dicts(a: dict[int, int], b: dict[int, int]) -> dict[int, int]:
    """Merge two dicts, second dict values win on conflict."""
    result: dict[int, int] = {}
    for k in a:
        result[k] = a[k]
    for k in b:
        result[k] = b[k]
    return result


def test_module() -> int:
    """Test all dict comprehension functions."""
    sq: dict[int, int] = squares_dict(4)
    assert sq[1] == 1
    assert sq[2] == 4
    assert sq[3] == 9
    assert sq[4] == 16
    inv: dict[int, int] = invert_dict(sq)
    assert inv[1] == 1
    assert inv[4] == 2
    assert inv[9] == 3
    counts: dict[int, int] = count_elements([1, 2, 2, 3, 3, 3])
    assert counts[1] == 1
    assert counts[2] == 2
    assert counts[3] == 3
    d1: dict[int, int] = {1: 10, 2: 20}
    d2: dict[int, int] = {2: 99, 3: 30}
    merged: dict[int, int] = merge_dicts(d1, d2)
    assert merged[1] == 10
    assert merged[2] == 99
    assert merged[3] == 30
    return 0


if __name__ == "__main__":
    test_module()
