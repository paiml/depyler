"""B-tree index simulation using flat arrays.

Simulates a B-tree with order 3 (max 2 keys per node) using parallel arrays.
Nodes stored as: keys[], children[], parent[], num_keys[].
"""


def btree_search(keys: list[int], children: list[int], num_keys: list[int], node: int, target: int, order: int) -> int:
    """Search for target in B-tree. Returns 1 if found, 0 otherwise.

    Each node has max (order-1) keys. keys[node*order..] are the node's keys.
    children[node*(order+1)..] are children indices (-1 = null).
    """
    nk: int = num_keys[node]
    i: int = 0
    while i < nk:
        kv: int = keys[node * order + i]
        if kv == target:
            return 1
        if target < kv:
            ci_offset: int = node * (order + 1) + i
            child_idx: int = children[ci_offset]
            if child_idx < 0:
                return 0
            sub_result: int = btree_search(keys, children, num_keys, child_idx, target, order)
            return sub_result
        i = i + 1
    ci_offset2: int = node * (order + 1) + nk
    child_idx2: int = children[ci_offset2]
    if child_idx2 < 0:
        return 0
    sub_result2: int = btree_search(keys, children, num_keys, child_idx2, target, order)
    return sub_result2


def sorted_insert(arr: list[int], val: int) -> list[int]:
    """Insert val into sorted array maintaining order."""
    result: list[int] = []
    inserted: int = 0
    i: int = 0
    while i < len(arr):
        av: int = arr[i]
        if inserted == 0:
            if val <= av:
                result.append(val)
                inserted = 1
        result.append(av)
        i = i + 1
    if inserted == 0:
        result.append(val)
    return result


def binary_search_sorted(arr: list[int], target: int) -> int:
    """Binary search in sorted array. Returns index or -1."""
    lo: int = 0
    hi: int = len(arr) - 1
    while lo <= hi:
        mid: int = (lo + hi) // 2
        mv: int = arr[mid]
        if mv == target:
            return mid
        if mv < target:
            lo = mid + 1
        else:
            hi = mid - 1
    return 0 - 1


def range_query(sorted_arr: list[int], lo_val: int, hi_val: int) -> list[int]:
    """Return all elements in [lo_val, hi_val] from sorted array."""
    result: list[int] = []
    i: int = 0
    while i < len(sorted_arr):
        sv: int = sorted_arr[i]
        if sv >= lo_val:
            if sv <= hi_val:
                result.append(sv)
        i = i + 1
    return result


def count_in_range(sorted_arr: list[int], lo_val: int, hi_val: int) -> int:
    """Count elements in [lo_val, hi_val]."""
    cnt: int = 0
    i: int = 0
    while i < len(sorted_arr):
        sv: int = sorted_arr[i]
        if sv >= lo_val:
            if sv <= hi_val:
                cnt = cnt + 1
        i = i + 1
    return cnt


def test_module() -> int:
    """Test B-tree index operations."""
    ok: int = 0
    keys: list[int] = [10, 20, 0, 5, 0, 0, 15, 0, 0, 25, 30, 0]
    children: list[int] = [1, 2, 3, 0 - 1, 0 - 1, 0 - 1, 0 - 1, 0 - 1, 0 - 1, 0 - 1, 0 - 1, 0 - 1, 0 - 1, 0 - 1, 0 - 1, 0 - 1]
    num_k: list[int] = [2, 1, 1, 2]
    found10: int = btree_search(keys, children, num_k, 0, 10, 3)
    if found10 == 1:
        ok = ok + 1
    found5: int = btree_search(keys, children, num_k, 0, 5, 3)
    if found5 == 1:
        ok = ok + 1
    ins: list[int] = sorted_insert([1, 3, 5], 4)
    v2: int = ins[2]
    if v2 == 4:
        ok = ok + 1
    bs: int = binary_search_sorted([1, 3, 5, 7, 9], 5)
    if bs == 2:
        ok = ok + 1
    rng: list[int] = range_query([1, 2, 3, 4, 5, 6, 7, 8], 3, 6)
    if len(rng) == 4:
        ok = ok + 1
    return ok
