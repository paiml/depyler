# Typed tuple operations for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def swap_pair(a: int, b: int) -> tuple[int, int]:
    """Swap two integers returning a tuple."""
    return (b, a)


def min_max(nums: list[int]) -> tuple[int, int]:
    """Return (min, max) of a list."""
    if len(nums) == 0:
        return (0, 0)
    lo: int = nums[0]
    hi: int = nums[0]
    for n in nums:
        if n < lo:
            lo = n
        if n > hi:
            hi = n
    return (lo, hi)


def divmod_result(a: int, b: int) -> tuple[int, int]:
    """Return (quotient, remainder) tuple."""
    if b == 0:
        return (0, 0)
    return (a // b, a % b)


def tuple_sum(t: tuple[int, int, int]) -> int:
    """Sum elements of a 3-tuple."""
    return t[0] + t[1] + t[2]


def make_triple(a: int, b: int, c: int) -> tuple[int, int, int]:
    """Create a sorted triple."""
    x: int = a
    y: int = b
    z: int = c
    if x > y:
        temp: int = x
        x = y
        y = temp
    if y > z:
        temp2: int = y
        y = z
        z = temp2
    if x > y:
        temp3: int = x
        x = y
        y = temp3
    return (x, y, z)


def test_module() -> int:
    """Test all tuple operations."""
    s: tuple[int, int] = swap_pair(3, 7)
    assert s[0] == 7
    assert s[1] == 3
    mm: tuple[int, int] = min_max([3, 1, 4, 1, 5, 9])
    assert mm[0] == 1
    assert mm[1] == 9
    dm: tuple[int, int] = divmod_result(17, 5)
    assert dm[0] == 3
    assert dm[1] == 2
    assert tuple_sum((10, 20, 30)) == 60
    t: tuple[int, int, int] = make_triple(5, 1, 3)
    assert t[0] == 1
    assert t[1] == 3
    assert t[2] == 5
    return 0


if __name__ == "__main__":
    test_module()
