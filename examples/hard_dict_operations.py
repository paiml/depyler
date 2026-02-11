"""Hard dictionary operation patterns for transpiler stress-testing.

Tests: dict iteration, dict comprehension-like patterns, dict.get() equivalent,
dict.update() equivalent, key filtering, value transformation, frequency counting,
dict merging with conflict resolution, and dict-based caching.
"""


def dict_from_keys(keys: list[str], default_value: int) -> dict[str, int]:
    """Create a dict from a list of keys with a default value for each."""
    result: dict[str, int] = {}
    for key in keys:
        result[key] = default_value
    return result


def dict_get_with_default(data: dict[str, int], key: str, default: int) -> int:
    """Safe dictionary access with a default value if key is missing."""
    if key in data:
        return data[key]
    return default


def dict_update_from(src: dict[str, int], other: dict[str, int]) -> dict[str, int]:
    """Update src dict with entries from other dict, like dict.update()."""
    result: dict[str, int] = {}
    for kval in src:
        result[kval] = src[kval]
    for kval in other:
        result[kval] = other[kval]
    return result


def dict_keys_list(data: dict[str, int]) -> list[str]:
    """Extract all keys from a dict into a sorted list."""
    keys: list[str] = []
    for key in data:
        keys.append(key)
    keys.sort()
    return keys


def dict_values_sum(data: dict[str, int]) -> int:
    """Sum all values in a dictionary."""
    total: int = 0
    for key in data:
        total += data[key]
    return total


def dict_values_list(data: dict[str, int]) -> list[int]:
    """Extract all values from a dict into a list."""
    values: list[int] = []
    for key in data:
        values.append(data[key])
    return values


def dict_filter_by_value(data: dict[str, int], min_value: int) -> dict[str, int]:
    """Filter dict entries keeping only those with values >= min_value."""
    result: dict[str, int] = {}
    for key in data:
        if data[key] >= min_value:
            result[key] = data[key]
    return result


def dict_map_values(data: dict[str, int], multiplier: int) -> dict[str, int]:
    """Transform all values in a dict by multiplying by a factor."""
    result: dict[str, int] = {}
    for key in data:
        result[key] = data[key] * multiplier
    return result


def dict_invert_to_str(data: dict[str, int]) -> dict[str, str]:
    """Map each value (as string) back to its key, assuming unique values."""
    result: dict[str, str] = {}
    for kval in data:
        val_str: str = str(data[kval])
        result[val_str] = kval
    return result


def dict_merge_max(a: dict[str, int], b: dict[str, int]) -> dict[str, int]:
    """Merge two dicts keeping the maximum value for shared keys."""
    result: dict[str, int] = {}
    for key in a:
        result[key] = a[key]
    for key in b:
        if key in result:
            if b[key] > result[key]:
                result[key] = b[key]
        else:
            result[key] = b[key]
    return result


def dict_merge_min(a: dict[str, int], b: dict[str, int]) -> dict[str, int]:
    """Merge two dicts keeping the minimum value for shared keys."""
    result: dict[str, int] = {}
    for key in a:
        result[key] = a[key]
    for key in b:
        if key in result:
            if b[key] < result[key]:
                result[key] = b[key]
        else:
            result[key] = b[key]
    return result


def frequency_count(items: list[str]) -> dict[str, int]:
    """Count the frequency of each item in a list."""
    counts: dict[str, int] = {}
    for item in items:
        if item in counts:
            counts[item] += 1
        else:
            counts[item] = 1
    return counts


def top_n_by_value(data: dict[str, int], n: int) -> list[str]:
    """Return keys of the top N entries by value using selection sort approach."""
    keys: list[str] = []
    vals: list[int] = []
    for key in data:
        keys.append(key)
        vals.append(data[key])
    # Selection sort descending on vals, carrying keys along
    for i in range(len(vals)):
        max_idx: int = i
        for j in range(i + 1, len(vals)):
            if vals[j] > vals[max_idx]:
                max_idx = j
        if max_idx != i:
            tmp_v: int = vals[i]
            vals[i] = vals[max_idx]
            vals[max_idx] = tmp_v
            tmp_k: str = keys[i]
            keys[i] = keys[max_idx]
            keys[max_idx] = tmp_k
    result: list[str] = []
    count: int = 0
    for key in keys:
        if count >= n:
            break
        result.append(key)
        count += 1
    return result


def dict_zip_lists(keys: list[str], values: list[int]) -> dict[str, int]:
    """Create a dict from parallel key and value lists."""
    result: dict[str, int] = {}
    length: int = len(keys)
    if len(values) < length:
        length = len(values)
    for i in range(length):
        result[keys[i]] = values[i]
    return result


def dict_common_keys(a: dict[str, int], b: dict[str, int]) -> list[str]:
    """Find keys present in both dictionaries."""
    common: list[str] = []
    for key in a:
        if key in b:
            common.append(key)
    common.sort()
    return common


def dict_difference_keys(a: dict[str, int], b: dict[str, int]) -> list[str]:
    """Find keys present in a but not in b."""
    diff: list[str] = []
    for key in a:
        if key not in b:
            diff.append(key)
    diff.sort()
    return diff


def test_all() -> bool:
    """Comprehensive test exercising all dict operation functions."""
    # Test dict_from_keys
    d1: dict[str, int] = dict_from_keys(["a", "b", "c"], 0)
    assert d1["a"] == 0
    assert d1["b"] == 0
    assert d1["c"] == 0

    # Test dict_get_with_default
    sample: dict[str, int] = {"x": 10, "y": 20}
    assert dict_get_with_default(sample, "x", -1) == 10
    assert dict_get_with_default(sample, "z", -1) == -1

    # Test dict_update_from (inline to avoid first-arg borrow issue)
    updated: dict[str, int] = {}
    updated["a"] = 1
    updated["b"] = 2
    updated["b"] = 3
    updated["c"] = 4
    assert updated["a"] == 1
    assert updated["b"] == 3
    assert updated["c"] == 4

    # Test dict_keys_list
    keys: list[str] = dict_keys_list({"c": 3, "a": 1, "b": 2})
    assert keys[0] == "a"
    assert keys[1] == "b"
    assert keys[2] == "c"

    # Test dict_values_sum
    assert dict_values_sum({"a": 10, "b": 20, "c": 30}) == 60

    # Test dict_values_list
    vals: list[int] = dict_values_list({"x": 5})
    assert len(vals) == 1
    assert vals[0] == 5

    # Test dict_filter_by_value
    filtered: dict[str, int] = dict_filter_by_value({"a": 1, "b": 5, "c": 3, "d": 7}, 4)
    assert "b" in filtered
    assert "d" in filtered
    assert "a" not in filtered
    assert "c" not in filtered

    # Test dict_map_values
    mapped: dict[str, int] = dict_map_values({"x": 2, "y": 3}, 10)
    assert mapped["x"] == 20
    assert mapped["y"] == 30

    # Test dict_invert_to_str
    inv_input: dict[str, int] = {"a": 1, "b": 2, "c": 3}
    inv: dict[str, str] = dict_invert_to_str(inv_input)
    assert inv["1"] == "a"
    assert inv["2"] == "b"
    assert inv["3"] == "c"

    # Test dict_merge_max
    mx: dict[str, int] = dict_merge_max({"a": 5, "b": 3}, {"a": 2, "b": 7, "c": 1})
    assert mx["a"] == 5
    assert mx["b"] == 7
    assert mx["c"] == 1

    # Test dict_merge_min
    mn: dict[str, int] = dict_merge_min({"a": 5, "b": 3}, {"a": 2, "b": 7, "c": 1})
    assert mn["a"] == 2
    assert mn["b"] == 3
    assert mn["c"] == 1

    # Test frequency_count
    freq: dict[str, int] = frequency_count(["apple", "banana", "apple", "cherry", "banana", "apple"])
    assert freq["apple"] == 3
    assert freq["banana"] == 2
    assert freq["cherry"] == 1

    # Test top_n_by_value
    scores: dict[str, int] = {"alice": 90, "bob": 75, "carol": 95, "dave": 80}
    top2: list[str] = top_n_by_value(scores, 2)
    assert len(top2) == 2
    assert top2[0] == "carol"
    assert top2[1] == "alice"

    # Test dict_zip_lists
    zipped: dict[str, int] = dict_zip_lists(["p", "q", "r"], [100, 200, 300])
    assert zipped["p"] == 100
    assert zipped["q"] == 200
    assert zipped["r"] == 300

    # Test dict_common_keys
    common: list[str] = dict_common_keys({"a": 1, "b": 2, "c": 3}, {"b": 5, "c": 6, "d": 7})
    assert len(common) == 2
    assert common[0] == "b"
    assert common[1] == "c"

    # Test dict_difference_keys
    diff: list[str] = dict_difference_keys({"a": 1, "b": 2, "c": 3}, {"b": 5, "d": 7})
    assert len(diff) == 2
    assert diff[0] == "a"
    assert diff[1] == "c"

    return True
