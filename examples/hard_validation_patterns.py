# Input validation patterns (bounds checking, type checking)
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def clamp_value(value: int, low: int, high: int) -> int:
    """Clamp a value to be within [low, high]."""
    if value < low:
        return low
    if value > high:
        return high
    return value


def is_valid_index(lst: list[int], idx: int) -> bool:
    """Check if an index is valid for the given list."""
    if idx < 0:
        return False
    if idx >= len(lst):
        return False
    return True


def safe_get(lst: list[int], idx: int, default: int) -> int:
    """Get element at index, returning default if out of bounds."""
    if idx < 0 or idx >= len(lst):
        return default
    return lst[idx]


def validate_positive(n: int) -> int:
    """Return n if positive, otherwise return 0."""
    if n > 0:
        return n
    return 0


def all_positive(nums: list[int]) -> bool:
    """Check if all elements are positive."""
    i: int = 0
    while i < len(nums):
        if nums[i] <= 0:
            return False
        i = i + 1
    return True


def test_module() -> int:
    assert clamp_value(5, 1, 10) == 5
    assert clamp_value(-5, 0, 100) == 0
    assert clamp_value(200, 0, 100) == 100
    data: list[int] = [10, 20, 30, 40, 50]
    assert is_valid_index(data, 0) == True
    assert is_valid_index(data, 4) == True
    assert is_valid_index(data, 5) == False
    assert is_valid_index(data, -1) == False
    assert safe_get(data, 2, -1) == 30
    assert safe_get(data, 10, -1) == -1
    assert safe_get(data, -1, 99) == 99
    assert validate_positive(5) == 5
    assert validate_positive(-3) == 0
    assert validate_positive(0) == 0
    assert all_positive([1, 2, 3]) == True
    assert all_positive([1, -2, 3]) == False
    assert all_positive([]) == True
    return 0


if __name__ == "__main__":
    test_module()
