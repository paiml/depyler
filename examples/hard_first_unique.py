"""First unique element finding in arrays and strings."""


def first_unique_int(arr: list[int]) -> int:
    """Find first element that appears exactly once. Returns -1 if none."""
    counts: dict[str, int] = {}
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        val_str: str = str(arr[idx])
        if val_str in counts:
            counts[val_str] = counts[val_str] + 1
        else:
            counts[val_str] = 1
        idx = idx + 1
    idx2: int = 0
    while idx2 < length:
        val_str2: str = str(arr[idx2])
        if counts[val_str2] == 1:
            return arr[idx2]
        idx2 = idx2 + 1
    return -1


def first_unique_char_index(s: str) -> int:
    """Find index of first non-repeating character. Returns -1 if none."""
    counts: dict[str, int] = {}
    idx: int = 0
    length: int = len(s)
    while idx < length:
        ch: str = s[idx]
        if ch in counts:
            counts[ch] = counts[ch] + 1
        else:
            counts[ch] = 1
        idx = idx + 1
    idx2: int = 0
    while idx2 < length:
        ch2: str = s[idx2]
        if counts[ch2] == 1:
            return idx2
        idx2 = idx2 + 1
    return -1


def last_unique_int(arr: list[int]) -> int:
    """Find last element that appears exactly once. Returns -1 if none."""
    counts: dict[str, int] = {}
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        val_str: str = str(arr[idx])
        if val_str in counts:
            counts[val_str] = counts[val_str] + 1
        else:
            counts[val_str] = 1
        idx = idx + 1
    result: int = -1
    idx2: int = 0
    while idx2 < length:
        val_str2: str = str(arr[idx2])
        if counts[val_str2] == 1:
            result = arr[idx2]
        idx2 = idx2 + 1
    return result


def count_unique_elements(arr: list[int]) -> int:
    """Count how many elements appear exactly once."""
    counts: dict[str, int] = {}
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        val_str: str = str(arr[idx])
        if val_str in counts:
            counts[val_str] = counts[val_str] + 1
        else:
            counts[val_str] = 1
        idx = idx + 1
    unique_count: int = 0
    idx2: int = 0
    seen: dict[str, int] = {}
    while idx2 < length:
        val_str2: str = str(arr[idx2])
        if val_str2 not in seen:
            if counts[val_str2] == 1:
                unique_count = unique_count + 1
            seen[val_str2] = 1
        idx2 = idx2 + 1
    return unique_count


def test_module() -> int:
    passed: int = 0

    if first_unique_int([2, 3, 2, 4, 3]) == 4:
        passed = passed + 1
    if first_unique_int([1, 1, 2, 2]) == -1:
        passed = passed + 1
    if first_unique_char_index("aabbc") == 4:
        passed = passed + 1
    if first_unique_char_index("aabb") == -1:
        passed = passed + 1
    if last_unique_int([1, 2, 3, 2, 1]) == 3:
        passed = passed + 1
    if count_unique_elements([1, 2, 2, 3, 3, 4]) == 2:
        passed = passed + 1
    if first_unique_int([5]) == 5:
        passed = passed + 1

    return passed
