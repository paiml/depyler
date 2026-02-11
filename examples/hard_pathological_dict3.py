# Pathological dict: Multiple dict operations in sequence
# Tests: create, update, delete, merge, compare dicts


def merge_dicts(a: dict[str, int], b: dict[str, int], merge_keys: list[str]) -> dict[str, int]:
    """Merge values from b into a for specified keys. Sum on collision."""
    result: dict[str, int] = {}
    # Copy a
    i: int = 0
    while i < len(merge_keys):
        mk: str = merge_keys[i]
        if mk in a:
            result[mk] = a[mk]
        i = i + 1
    # Merge b
    j: int = 0
    while j < len(merge_keys):
        mk2: str = merge_keys[j]
        if mk2 in b:
            if mk2 in result:
                result[mk2] = result[mk2] + b[mk2]
            else:
                result[mk2] = b[mk2]
        j = j + 1
    return result


def dict_difference(a: dict[str, int], b: dict[str, int], check_keys: list[str]) -> list[str]:
    """Find keys in check_keys where a[k] != b[k] or key missing from one."""
    diffs: list[str] = []
    i: int = 0
    while i < len(check_keys):
        ck: str = check_keys[i]
        a_has: bool = ck in a
        b_has: bool = ck in b
        if a_has == True and b_has == True:
            if a[ck] != b[ck]:
                diffs.append(ck)
        elif a_has != b_has:
            diffs.append(ck)
        i = i + 1
    return diffs


def invert_dict(d: dict[str, int], keys_list: list[str]) -> dict[int, int]:
    """Invert dict: values become keys, count occurrences."""
    inv: dict[int, int] = {}
    i: int = 0
    while i < len(keys_list):
        k: str = keys_list[i]
        if k in d:
            v: int = d[k]
            if v in inv:
                inv[v] = inv[v] + 1
            else:
                inv[v] = 1
        i = i + 1
    return inv


def top_n_values(d: dict[str, int], keys_list: list[str], n: int) -> list[int]:
    """Get top n values from dict (sorted descending). Simple selection."""
    all_vals: list[int] = []
    i: int = 0
    while i < len(keys_list):
        k: str = keys_list[i]
        if k in d:
            all_vals.append(d[k])
        i = i + 1
    # Sort descending via repeated extraction of max
    result: list[int] = []
    collected: int = 0
    while collected < n and len(all_vals) > 0:
        max_val: int = all_vals[0]
        max_idx: int = 0
        j: int = 1
        while j < len(all_vals):
            if all_vals[j] > max_val:
                max_val = all_vals[j]
                max_idx = j
            j = j + 1
        result.append(max_val)
        # Remove max_idx via rebuild
        new_vals: list[int] = []
        k2: int = 0
        while k2 < len(all_vals):
            if k2 != max_idx:
                new_vals.append(all_vals[k2])
            k2 = k2 + 1
        all_vals = new_vals
        collected = collected + 1
    return result


def test_module() -> int:
    passed: int = 0
    a: dict[str, int] = {"x": 10, "y": 20}
    b: dict[str, int] = {"y": 5, "z": 30}
    merge_keys: list[str] = ["x", "y", "z"]
    # Test 1: merge
    merged: dict[str, int] = merge_dicts(a, b, merge_keys)
    if merged["y"] == 25:
        passed = passed + 1
    # Test 2: merge preserves a-only
    if merged["x"] == 10:
        passed = passed + 1
    # Test 3: merge preserves b-only
    if merged["z"] == 30:
        passed = passed + 1
    # Test 4: difference
    d1: dict[str, int] = {"a": 1, "b": 2, "c": 3}
    d2: dict[str, int] = {"a": 1, "b": 99, "c": 3}
    diffs: list[str] = dict_difference(d1, d2, ["a", "b", "c"])
    if len(diffs) == 1:
        passed = passed + 1
    # Test 5: invert
    inv: dict[int, int] = invert_dict(d1, ["a", "b", "c"])
    if inv[1] == 1:
        passed = passed + 1
    # Test 6: top_n
    scores: dict[str, int] = {"alice": 90, "bob": 85, "carol": 95}
    top2: list[int] = top_n_values(scores, ["alice", "bob", "carol"], 2)
    if top2[0] == 95:
        passed = passed + 1
    # Test 7: top_n second
    if top2[1] == 90:
        passed = passed + 1
    return passed
