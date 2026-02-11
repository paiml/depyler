"""Two sum problem variants using dict-based lookups."""


def two_sum_indices(arr: list[int], target: int) -> list[int]:
    """Find indices of two numbers that add up to target. Returns [-1, -1] if none."""
    seen: dict[str, int] = {}
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        complement: int = target - arr[idx]
        comp_str: str = str(complement)
        if comp_str in seen:
            result: list[int] = [seen[comp_str], idx]
            return result
        val_str: str = str(arr[idx])
        seen[val_str] = idx
        idx = idx + 1
    not_found: list[int] = [-1, -1]
    return not_found


def two_sum_exists(arr: list[int], target: int) -> int:
    """Return 1 if any pair sums to target, else 0."""
    seen: dict[str, int] = {}
    idx: int = 0
    length: int = len(arr)
    while idx < length:
        complement: int = target - arr[idx]
        comp_str: str = str(complement)
        if comp_str in seen:
            return 1
        val_str: str = str(arr[idx])
        seen[val_str] = 1
        idx = idx + 1
    return 0


def count_pairs_with_sum(arr: list[int], target: int) -> int:
    """Count number of pairs that sum to target."""
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

    total: int = 0
    idx2: int = 0
    while idx2 < length:
        complement: int = target - arr[idx2]
        comp_str: str = str(complement)
        if comp_str in counts:
            total = total + counts[comp_str]
            if complement == arr[idx2]:
                total = total - 1
        idx2 = idx2 + 1
    return total // 2


def test_module() -> int:
    passed: int = 0

    result: list[int] = two_sum_indices([2, 7, 11, 15], 9)
    if result[0] == 0:
        passed = passed + 1
    if result[1] == 1:
        passed = passed + 1

    none_res: list[int] = two_sum_indices([1, 2, 3], 100)
    if none_res[0] == -1:
        passed = passed + 1

    if two_sum_exists([1, 5, 3, 7], 8) == 1:
        passed = passed + 1
    if two_sum_exists([1, 2, 3], 100) == 0:
        passed = passed + 1

    if count_pairs_with_sum([1, 5, 7, 1], 6) == 2:
        passed = passed + 1

    if two_sum_exists([0, 0], 0) == 1:
        passed = passed + 1

    return passed
