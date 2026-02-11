"""Boolean flag patterns and state tracking with int flags."""


def all_positive(arr: list[int]) -> int:
    """Return 1 if all elements are positive."""
    found_non_pos: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] <= 0:
            found_non_pos = 1
        i = i + 1
    if found_non_pos == 0:
        return 1
    return 0


def any_negative(arr: list[int]) -> int:
    """Return 1 if any element is negative."""
    found: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] < 0:
            found = 1
        i = i + 1
    return found


def is_sorted_asc(arr: list[int]) -> int:
    """Return 1 if array is sorted ascending."""
    if len(arr) <= 1:
        return 1
    ok: int = 1
    i: int = 1
    while i < len(arr):
        if arr[i] < arr[i - 1]:
            ok = 0
        i = i + 1
    return ok


def has_consecutive_duplicates(arr: list[int]) -> int:
    """Return 1 if any two consecutive elements are equal."""
    if len(arr) <= 1:
        return 0
    found: int = 0
    i: int = 1
    while i < len(arr):
        if arr[i] == arr[i - 1]:
            found = 1
        i = i + 1
    return found


def count_transitions(arr: list[int]) -> int:
    """Count number of times the sign changes in the array."""
    if len(arr) <= 1:
        return 0
    transitions: int = 0
    i: int = 1
    while i < len(arr):
        prev_pos: int = 0
        if arr[i - 1] > 0:
            prev_pos = 1
        curr_pos: int = 0
        if arr[i] > 0:
            curr_pos = 1
        if prev_pos != curr_pos:
            transitions = transitions + 1
        i = i + 1
    return transitions


def track_min_max(arr: list[int]) -> list[int]:
    """Track running min and max, return [final_min, final_max, range]."""
    if len(arr) == 0:
        return [0, 0, 0]
    min_val: int = arr[0]
    max_val: int = arr[0]
    i: int = 1
    while i < len(arr):
        if arr[i] < min_val:
            min_val = arr[i]
        if arr[i] > max_val:
            max_val = arr[i]
        i = i + 1
    return [min_val, max_val, max_val - min_val]


def validate_brackets(depths: list[int]) -> int:
    """Simulate bracket validation using depth tracking.
    Each element is +1 (open) or -1 (close).
    Return 1 if valid (depth never negative, ends at 0)."""
    depth: int = 0
    went_negative: int = 0
    i: int = 0
    while i < len(depths):
        depth = depth + depths[i]
        if depth < 0:
            went_negative = 1
        i = i + 1
    if went_negative == 1:
        return 0
    if depth != 0:
        return 0
    return 1


def longest_streak(arr: list[int], target: int) -> int:
    """Find longest consecutive run of target value."""
    best: int = 0
    current: int = 0
    i: int = 0
    while i < len(arr):
        if arr[i] == target:
            current = current + 1
            if current > best:
                best = current
        else:
            current = 0
        i = i + 1
    return best


def first_repeated_index(arr: list[int]) -> int:
    """Find index of first element that appeared before. Returns -1 if none."""
    seen: dict[int, int] = {}
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        if val in seen:
            return i
        seen[val] = i
        i = i + 1
    return -1


def test_module() -> int:
    """Test all flag pattern functions."""
    passed: int = 0
    if all_positive([1, 2, 3]) == 1:
        passed = passed + 1
    if all_positive([1, 0, 3]) == 0:
        passed = passed + 1
    if any_negative([1, 0 - 1, 3]) == 1:
        passed = passed + 1
    if any_negative([1, 2, 3]) == 0:
        passed = passed + 1
    if is_sorted_asc([1, 2, 3, 4]) == 1:
        passed = passed + 1
    if is_sorted_asc([1, 3, 2]) == 0:
        passed = passed + 1
    if has_consecutive_duplicates([1, 2, 2, 3]) == 1:
        passed = passed + 1
    if has_consecutive_duplicates([1, 2, 3]) == 0:
        passed = passed + 1
    ct: int = count_transitions([1, 0 - 1, 1, 0 - 1])
    if ct == 3:
        passed = passed + 1
    mm: list[int] = track_min_max([3, 1, 4, 1, 5])
    if mm[0] == 1:
        passed = passed + 1
    if mm[1] == 5:
        passed = passed + 1
    if validate_brackets([1, 1, 0 - 1, 0 - 1]) == 1:
        passed = passed + 1
    if validate_brackets([1, 0 - 1, 0 - 1, 1]) == 0:
        passed = passed + 1
    if longest_streak([1, 1, 2, 1, 1, 1], 1) == 3:
        passed = passed + 1
    if first_repeated_index([5, 3, 7, 3, 9]) == 3:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
