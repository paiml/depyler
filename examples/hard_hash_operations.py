# HashMap operations (merge, filter, transform)
# NO imports, NO I/O, ALL pure functions, ALL type-annotated


def dict_merge(a: dict[str, int], b: dict[str, int]) -> dict[str, int]:
    """Merge two dicts, with b overriding a on conflicts."""
    result: dict[str, int] = {}
    for k in a:
        result[k] = a[k]
    for k in b:
        result[k] = b[k]
    return result


def dict_sum_values(d: dict[str, int]) -> int:
    """Sum all values in a dict."""
    total: int = 0
    for k in d:
        total = total + d[k]
    return total


def dict_count_keys(d: dict[str, int]) -> int:
    """Count the number of keys in a dict."""
    count: int = 0
    for k in d:
        count = count + 1
    return count


def dict_max_value(d: dict[str, int]) -> int:
    """Find the maximum value in a dict. Returns 0 if empty."""
    result: int = 0
    first: bool = True
    for k in d:
        if first:
            result = d[k]
            first = False
        elif d[k] > result:
            result = d[k]
    return result


def test_module() -> int:
    a: dict[str, int] = {"x": 1, "y": 2}
    b: dict[str, int] = {"y": 10, "z": 3}
    merged: dict[str, int] = dict_merge(a, b)
    assert merged["x"] == 1
    assert merged["y"] == 10
    assert merged["z"] == 3
    assert dict_sum_values({"a": 1, "b": 2, "c": 3}) == 6
    empty: dict[str, int] = {"_": 0}
    assert dict_count_keys({"a": 1, "b": 2}) == 2
    assert dict_max_value({"a": 5, "b": 3, "c": 8}) == 8
    assert dict_max_value({"a": 1}) == 1
    return 0


if __name__ == "__main__":
    test_module()
