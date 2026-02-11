"""Map, filter, reduce patterns implemented using loops."""


def map_double(arr: list[int]) -> list[int]:
    """Double each element."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i] * 2)
        i = i + 1
    return result


def map_square(arr: list[int]) -> list[int]:
    """Square each element."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        result.append(arr[i] * arr[i])
        i = i + 1
    return result


def map_abs(arr: list[int]) -> list[int]:
    """Absolute value of each element."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        if val < 0:
            val = 0 - val
        result.append(val)
        i = i + 1
    return result


def filter_positive(arr: list[int]) -> list[int]:
    """Keep only positive elements."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        if arr[i] > 0:
            result.append(arr[i])
        i = i + 1
    return result


def filter_even(arr: list[int]) -> list[int]:
    """Keep only even elements."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        if arr[i] % 2 == 0:
            result.append(arr[i])
        i = i + 1
    return result


def filter_range(arr: list[int], lo: int, hi: int) -> list[int]:
    """Keep elements in [lo, hi] range."""
    result: list[int] = []
    i: int = 0
    while i < len(arr):
        if arr[i] >= lo and arr[i] <= hi:
            result.append(arr[i])
        i = i + 1
    return result


def reduce_sum(arr: list[int]) -> int:
    """Sum all elements."""
    total: int = 0
    i: int = 0
    while i < len(arr):
        total = total + arr[i]
        i = i + 1
    return total


def reduce_product(arr: list[int]) -> int:
    """Product of all elements."""
    if len(arr) == 0:
        return 0
    product: int = 1
    i: int = 0
    while i < len(arr):
        product = product * arr[i]
        i = i + 1
    return product


def reduce_max(arr: list[int]) -> int:
    """Find maximum element."""
    if len(arr) == 0:
        return 0
    best: int = arr[0]
    i: int = 1
    while i < len(arr):
        if arr[i] > best:
            best = arr[i]
        i = i + 1
    return best


def zip_add(a: list[int], b: list[int]) -> list[int]:
    """Element-wise addition of two lists."""
    result: list[int] = []
    n: int = len(a)
    if len(b) < n:
        n = len(b)
    i: int = 0
    while i < n:
        result.append(a[i] + b[i])
        i = i + 1
    return result


def chain_map_filter(arr: list[int]) -> list[int]:
    """Map (square), then filter (> 10), then map (subtract 10)."""
    step1: list[int] = map_square(arr)
    step2: list[int] = []
    i: int = 0
    while i < len(step1):
        if step1[i] > 10:
            step2.append(step1[i])
        i = i + 1
    step3: list[int] = []
    i = 0
    while i < len(step2):
        step3.append(step2[i] - 10)
        i = i + 1
    return step3


def test_module() -> int:
    """Test all functional pattern functions."""
    passed: int = 0
    if map_double([1, 2, 3]) == [2, 4, 6]:
        passed = passed + 1
    if map_square([1, 2, 3]) == [1, 4, 9]:
        passed = passed + 1
    if map_abs([0 - 1, 2, 0 - 3]) == [1, 2, 3]:
        passed = passed + 1
    if filter_positive([0 - 1, 0, 1, 2]) == [1, 2]:
        passed = passed + 1
    if filter_even([1, 2, 3, 4, 5]) == [2, 4]:
        passed = passed + 1
    if filter_range([1, 5, 3, 8, 2], 2, 5) == [5, 3, 2]:
        passed = passed + 1
    if reduce_sum([1, 2, 3, 4]) == 10:
        passed = passed + 1
    if reduce_product([1, 2, 3, 4]) == 24:
        passed = passed + 1
    if reduce_max([3, 1, 4, 1, 5]) == 5:
        passed = passed + 1
    za: list[int] = zip_add([1, 2, 3], [4, 5, 6])
    if za == [5, 7, 9]:
        passed = passed + 1
    ch: list[int] = chain_map_filter([1, 2, 3, 4])
    if ch[0] == 6:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
