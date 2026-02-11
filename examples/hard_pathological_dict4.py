# Pathological dict: Dict with string keys and int values - complex lookups
# Tests: multi-level lookups, conditional dict operations, dict as config


def create_config(items: list[str], vals: list[int]) -> dict[str, int]:
    """Create config dict from parallel arrays."""
    cfg: dict[str, int] = {}
    i: int = 0
    limit: int = len(items)
    if len(vals) < limit:
        limit = len(vals)
    while i < limit:
        cfg[items[i]] = vals[i]
        i = i + 1
    return cfg


def lookup_with_default(d: dict[str, int], name: str, fallback: int) -> int:
    """Lookup key in dict, return fallback if not found."""
    if name in d:
        return d[name]
    return fallback


def apply_multipliers(vals: list[int], multipliers: dict[int, int], indices: list[int]) -> list[int]:
    """Apply multiplier from dict to each value by its index."""
    result: list[int] = []
    i: int = 0
    while i < len(vals):
        idx: int = indices[i]
        if idx in multipliers:
            result.append(vals[i] * multipliers[idx])
        else:
            result.append(vals[i])
        i = i + 1
    return result


def count_matching_keys(d1: dict[str, int], d2: dict[str, int], keys_to_check: list[str]) -> int:
    """Count how many keys have the same value in both dicts."""
    matches: int = 0
    i: int = 0
    while i < len(keys_to_check):
        k: str = keys_to_check[i]
        if k in d1 and k in d2:
            if d1[k] == d2[k]:
                matches = matches + 1
        i = i + 1
    return matches


def dict_to_sorted_values(d: dict[str, int], ordered_keys: list[str]) -> list[int]:
    """Extract values from dict in order of provided keys."""
    result: list[int] = []
    i: int = 0
    while i < len(ordered_keys):
        k: str = ordered_keys[i]
        if k in d:
            result.append(d[k])
        else:
            result.append(0)
        i = i + 1
    return result


def test_module() -> int:
    passed: int = 0
    # Test 1: create config
    cfg: dict[str, int] = create_config(["width", "height", "depth"], [100, 200, 50])
    if cfg["width"] == 100:
        passed = passed + 1
    # Test 2: lookup with default
    if lookup_with_default(cfg, "height", 0) == 200:
        passed = passed + 1
    # Test 3: missing key default
    if lookup_with_default(cfg, "color", 999) == 999:
        passed = passed + 1
    # Test 4: apply multipliers
    mults: dict[int, int] = {0: 2, 1: 3}
    applied: list[int] = apply_multipliers([10, 20, 30], mults, [0, 1, 2])
    if applied[0] == 20 and applied[1] == 60 and applied[2] == 30:
        passed = passed + 1
    # Test 5: count matching keys
    d1: dict[str, int] = {"a": 1, "b": 2, "c": 3}
    d2: dict[str, int] = {"a": 1, "b": 99, "c": 3}
    if count_matching_keys(d1, d2, ["a", "b", "c"]) == 2:
        passed = passed + 1
    # Test 6: dict to sorted values
    scores: dict[str, int] = {"math": 90, "eng": 85, "sci": 95}
    vals: list[int] = dict_to_sorted_values(scores, ["eng", "math", "sci"])
    if vals[0] == 85 and vals[1] == 90 and vals[2] == 95:
        passed = passed + 1
    # Test 7: empty config
    empty_cfg: dict[str, int] = create_config([], [])
    if len(empty_cfg) == 0:
        passed = passed + 1
    return passed
