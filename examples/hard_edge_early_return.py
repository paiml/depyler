"""Functions with many early return paths."""


def classify_number(n: int) -> int:
    """Classify number into categories with early returns.
    Returns: 0=zero, 1=one, 2=prime<10, 3=even, 4=odd, 5=large."""
    if n == 0:
        return 0
    if n == 1:
        return 1
    if n == 2:
        return 2
    if n == 3:
        return 2
    if n == 5:
        return 2
    if n == 7:
        return 2
    if n > 1000:
        return 5
    if n % 2 == 0:
        return 3
    return 4


def validate_range(val: int, lo: int, hi: int) -> int:
    """Validate with multiple early returns."""
    if val < lo:
        return -1
    if val > hi:
        return 1
    if val == lo:
        return -2
    if val == hi:
        return 2
    mid: int = (lo + hi) // 2
    if val == mid:
        return 0
    if val < mid:
        return -3
    return 3


def safe_array_access(arr: list[int], idx: int) -> int:
    """Safe array access with early returns."""
    if len(arr) == 0:
        return -1
    if idx < 0:
        return -1
    if idx >= len(arr):
        return -1
    return arr[idx]


def find_first_match(arr: list[int], targets: list[int]) -> int:
    """Find index of first element that matches any target."""
    if len(arr) == 0:
        return -1
    if len(targets) == 0:
        return -1
    i: int = 0
    while i < len(arr):
        j: int = 0
        while j < len(targets):
            if arr[i] == targets[j]:
                return i
            j = j + 1
        i = i + 1
    return -1


def chain_classify(a: int, b: int, c: int) -> int:
    """Classify three values with cascading early returns."""
    if a == 0 and b == 0 and c == 0:
        return 0
    if a < 0:
        return -1
    if b < 0:
        return -2
    if c < 0:
        return -3
    total: int = a + b + c
    if total > 100:
        return 100
    if a > b and a > c:
        return 1
    if b > a and b > c:
        return 2
    if c > a and c > b:
        return 3
    return 4


def binary_search_with_guards(arr: list[int], target: int) -> int:
    """Binary search with many early-exit guards."""
    n: int = len(arr)
    if n == 0:
        return -1
    if target < arr[0]:
        return -1
    last_idx: int = n - 1
    if target > arr[last_idx]:
        return -1
    if arr[0] == target:
        return 0
    if arr[last_idx] == target:
        return last_idx
    lo: int = 0
    hi: int = last_idx
    while lo <= hi:
        mid: int = (lo + hi) // 2
        if arr[mid] == target:
            return mid
        if arr[mid] < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return -1


def test_module() -> int:
    """Test all early return functions."""
    passed: int = 0
    if classify_number(0) == 0:
        passed = passed + 1
    if classify_number(7) == 2:
        passed = passed + 1
    if classify_number(10) == 3:
        passed = passed + 1
    if classify_number(11) == 4:
        passed = passed + 1
    if classify_number(9999) == 5:
        passed = passed + 1
    if validate_range(5, 1, 10) == 0:
        passed = passed + 1
    if validate_range(0, 1, 10) == -1:
        passed = passed + 1
    if safe_array_access([10, 20, 30], 1) == 20:
        passed = passed + 1
    if safe_array_access([], 0) == -1:
        passed = passed + 1
    fm: int = find_first_match([5, 3, 8, 1], [8, 1])
    if fm == 2:
        passed = passed + 1
    if chain_classify(10, 5, 3) == 1:
        passed = passed + 1
    bs: int = binary_search_with_guards([1, 3, 5, 7, 9], 5)
    if bs == 2:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
