# Early return patterns with guard clauses for transpiler stress testing
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def safe_divide(a: int, b: int) -> int:
    """Divide with guard clause for zero divisor."""
    if b == 0:
        return 0
    return a // b


def find_in_list(nums: list[int], target: int) -> int:
    """Find target in list, return index or -1."""
    if len(nums) == 0:
        return -1
    i: int = 0
    while i < len(nums):
        if nums[i] == target:
            return i
        i = i + 1
    return -1


def is_prime(n: int) -> bool:
    """Check if n is prime with early returns."""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    i: int = 3
    while i * i <= n:
        if n % i == 0:
            return False
        i = i + 2
    return True


def validate_range(value: int, min_val: int, max_val: int) -> int:
    """Return value if in range, otherwise return boundary.
    Returns -1 for invalid range."""
    if min_val > max_val:
        return -1
    if value < min_val:
        return min_val
    if value > max_val:
        return max_val
    return value


def first_duplicate(nums: list[int]) -> int:
    """Find first value that appears more than once. Return -1 if none."""
    if len(nums) < 2:
        return -1
    i: int = 0
    while i < len(nums):
        j: int = i + 1
        while j < len(nums):
            if nums[i] == nums[j]:
                return nums[i]
            j = j + 1
        i = i + 1
    return -1


def test_module() -> int:
    """Test all early return pattern functions."""
    assert safe_divide(10, 3) == 3
    assert safe_divide(10, 0) == 0
    assert safe_divide(0, 5) == 0
    assert find_in_list([10, 20, 30, 40], 30) == 2
    assert find_in_list([10, 20, 30], 99) == -1
    assert find_in_list([], 1) == -1
    assert is_prime(2) == True
    assert is_prime(3) == True
    assert is_prime(4) == False
    assert is_prime(17) == True
    assert is_prime(1) == False
    assert is_prime(97) == True
    assert validate_range(5, 1, 10) == 5
    assert validate_range(-5, 0, 100) == 0
    assert validate_range(200, 0, 100) == 100
    assert validate_range(5, 10, 1) == -1
    assert first_duplicate([1, 2, 3, 2, 4]) == 2
    assert first_duplicate([1, 2, 3, 4]) == -1
    assert first_duplicate([5]) == -1
    return 0


if __name__ == "__main__":
    test_module()
