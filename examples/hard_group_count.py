"""Group elements and count occurrences using dictionaries."""


def count_occurrences(arr: list[int]) -> dict[str, int]:
    """Count occurrences of each element. Keys are string representations."""
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
    return counts


def most_frequent(arr: list[int]) -> int:
    """Find the most frequent element. On ties, return the first encountered."""
    counts: dict[str, int] = {}
    best_val: int = arr[0]
    best_count: int = 0
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        val_str: str = str(arr[idx])
        if val_str in counts:
            counts[val_str] = counts[val_str] + 1
        else:
            counts[val_str] = 1
        if counts[val_str] > best_count:
            best_count = counts[val_str]
            best_val = arr[idx]
        idx = idx + 1
    return best_val


def count_unique(arr: list[int]) -> int:
    """Count number of unique elements."""
    seen: dict[str, int] = {}
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        val_str: str = str(arr[idx])
        seen[val_str] = 1
        idx = idx + 1
    count: int = 0
    idx2: int = 0
    while idx2 < length:
        val_str2: str = str(arr[idx2])
        if val_str2 in seen:
            count = count + 1
            del seen[val_str2]
        idx2 = idx2 + 1
    return count


def elements_appearing_n_times(arr: list[int], n: int) -> list[int]:
    """Return elements appearing exactly n times."""
    counts: dict[str, int] = count_occurrences(arr)
    result: list[int] = []
    seen_already: dict[str, int] = {}
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        val_str: str = str(arr[idx])
        if val_str not in seen_already:
            if val_str in counts and counts[val_str] == n:
                result.append(arr[idx])
            seen_already[val_str] = 1
        idx = idx + 1
    return result


def test_module() -> int:
    passed: int = 0

    counts: dict[str, int] = count_occurrences([1, 2, 2, 3, 3, 3])
    if counts["1"] == 1:
        passed = passed + 1
    if counts["3"] == 3:
        passed = passed + 1

    if most_frequent([1, 2, 2, 3, 3, 3]) == 3:
        passed = passed + 1

    if count_unique([1, 2, 2, 3, 3, 3]) == 3:
        passed = passed + 1

    singles: list[int] = elements_appearing_n_times([1, 2, 2, 3, 3, 3], 1)
    if len(singles) == 1:
        passed = passed + 1

    doubles: list[int] = elements_appearing_n_times([1, 2, 2, 3, 3, 3], 2)
    if doubles[0] == 2:
        passed = passed + 1

    if count_unique([5, 5, 5]) == 1:
        passed = passed + 1

    return passed
