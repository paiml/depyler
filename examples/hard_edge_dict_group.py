"""Grouping by key and bucket operations using dicts."""


def group_by_remainder(arr: list[int], divisor: int) -> dict[int, list[int]]:
    """Group elements by their remainder when divided by divisor."""
    groups: dict[int, list[int]] = {}
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        rem: int = val % divisor
        if rem in groups:
            groups[rem].append(val)
        else:
            groups[rem] = [val]
        i = i + 1
    return groups


def group_by_sign(arr: list[int]) -> dict[int, list[int]]:
    """Group elements as negative (key -1), zero (key 0), positive (key 1)."""
    groups: dict[int, list[int]] = {}
    groups[0 - 1] = []
    groups[0] = []
    groups[1] = []
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        if val < 0:
            groups[0 - 1].append(val)
        elif val == 0:
            groups[0].append(val)
        else:
            groups[1].append(val)
        i = i + 1
    return groups


def bucket_sort_simple(arr: list[int], num_buckets: int, max_val: int) -> list[int]:
    """Simple bucket sort using dict of lists."""
    if len(arr) == 0:
        return []
    buckets: dict[int, list[int]] = {}
    b: int = 0
    while b < num_buckets:
        buckets[b] = []
        b = b + 1
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        bucket_idx: int = (val * num_buckets) // (max_val + 1)
        if bucket_idx >= num_buckets:
            bucket_idx = num_buckets - 1
        if bucket_idx < 0:
            bucket_idx = 0
        buckets[bucket_idx].append(val)
        i = i + 1
    b = 0
    while b < num_buckets:
        bkt: list[int] = buckets[b]
        j: int = 1
        while j < len(bkt):
            item: int = bkt[j]
            k: int = j - 1
            while k >= 0 and bkt[k] > item:
                bkt[k + 1] = bkt[k]
                k = k - 1
            bkt[k + 1] = item
            j = j + 1
        b = b + 1
    result: list[int] = []
    b = 0
    while b < num_buckets:
        bkt2: list[int] = buckets[b]
        j = 0
        while j < len(bkt2):
            result.append(bkt2[j])
            j = j + 1
        b = b + 1
    return result


def index_by_value(arr: list[int]) -> dict[int, list[int]]:
    """Create an inverted index: value -> list of positions."""
    index_map: dict[int, list[int]] = {}
    i: int = 0
    while i < len(arr):
        val: int = arr[i]
        if val in index_map:
            index_map[val].append(i)
        else:
            index_map[val] = [i]
        i = i + 1
    return index_map


def group_consecutive(arr: list[int]) -> list[list[int]]:
    """Group consecutive equal elements."""
    if len(arr) == 0:
        return []
    groups: list[list[int]] = []
    first_val: int = arr[0]
    current: list[int] = [first_val]
    i: int = 1
    while i < len(arr):
        if arr[i] == arr[i - 1]:
            current.append(arr[i])
        else:
            groups.append(current)
            new_val: int = arr[i]
            current = [new_val]
        i = i + 1
    groups.append(current)
    return groups


def test_module() -> int:
    """Test all grouping functions."""
    passed: int = 0
    g1: dict[int, list[int]] = group_by_remainder([1, 2, 3, 4, 5, 6], 3)
    if len(g1[0]) == 2:
        passed = passed + 1
    if len(g1[1]) == 2:
        passed = passed + 1
    g2: dict[int, list[int]] = group_by_sign([0 - 3, 0, 5, 0 - 1, 0, 7])
    neg_list: list[int] = g2[0 - 1]
    if len(neg_list) == 2:
        passed = passed + 1
    pos_list: list[int] = g2[1]
    if len(pos_list) == 2:
        passed = passed + 1
    sorted_arr: list[int] = bucket_sort_simple([5, 3, 8, 1, 7, 2], 3, 9)
    if sorted_arr == [1, 2, 3, 5, 7, 8]:
        passed = passed + 1
    idx: dict[int, list[int]] = index_by_value([10, 20, 10, 30, 20])
    idx_10: list[int] = idx[10]
    if len(idx_10) == 2:
        passed = passed + 1
    gc: list[list[int]] = group_consecutive([1, 1, 2, 2, 2, 3])
    if len(gc) == 3:
        passed = passed + 1
    first_group: list[int] = gc[0]
    if len(first_group) == 2:
        passed = passed + 1
    return passed


if __name__ == "__main__":
    print(test_module())
