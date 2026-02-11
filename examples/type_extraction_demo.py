"""
Type Extraction Demonstration.

This example showcases various Python type annotations that
Depyler's type extraction system can handle during transpilation.
Rewritten to use transpiler-friendly patterns: no imports, no classes,
no generics, no TypeVar, no Optional, no Union. Uses pure functions
with explicit types.
"""


def simple_types_test(a: int, b: float, c: str, d: int) -> int:
    """Function with simple type annotations."""
    return a


def container_list_test(items: list[int]) -> int:
    """Function with list type annotation. Returns sum."""
    total: int = 0
    for item in items:
        total = total + item
    return total


def container_dict_test(mapping: dict[str, str]) -> str:
    """Function with dict type annotation. Returns value for key."""
    result: str = mapping["key"]
    return result


def list_to_str_list(items: list[int]) -> list[str]:
    """Convert list of ints to list of strings."""
    result: list[str] = []
    for item in items:
        s: str = str(item)
        result.append(s)
    return result


def maybe_to_str(value: int) -> str:
    """Convert value to string, return empty if zero (simulating Optional)."""
    if value == 0:
        return ""
    result: str = str(value)
    return result


def nested_list_sum(matrix: list[list[int]]) -> int:
    """Sum all elements in a nested list."""
    total: int = 0
    for row in matrix:
        for item in row:
            total = total + item
    return total


def lookup_value(data: dict[str, str], key: str) -> str:
    """Look up a value in a dict, return empty string if missing."""
    cnt: int = 0
    for k in data:
        if k == key:
            cnt = cnt + 1
    if cnt > 0:
        result: str = data[key]
        return result
    return ""


def first_element(items: list[int]) -> int:
    """Return first element of list, or zero if empty."""
    if len(items) == 0:
        return 0
    return items[0]


def make_pair(a: int, b: int) -> list[int]:
    """Create a pair as a two-element list."""
    result: list[int] = []
    result.append(a)
    result.append(b)
    return result


def apply_add(a: int, b: int) -> int:
    """Higher-order function simulation: add."""
    result: int = a + b
    return result


def apply_mul(a: int, b: int) -> int:
    """Higher-order function simulation: multiply."""
    result: int = a * b
    return result


def apply_operation(a: int, b: int, op: int) -> int:
    """Dispatch to operation: 1=add, 2=mul."""
    if op == 1:
        return apply_add(a, b)
    if op == 2:
        return apply_mul(a, b)
    return 0


def user_lookup(user_ids: list[int], user_names: list[str], target_id: int) -> str:
    """Simulate typed alias lookup: find username by id."""
    i: int = 0
    while i < len(user_ids):
        if user_ids[i] == target_id:
            return user_names[i]
        i = i + 1
    return ""


def int_to_bool(val: int) -> int:
    """Convert int to boolean-like: 0->0, nonzero->1."""
    if val == 0:
        return 0
    return 1


def str_len_check(text: str, min_len: int) -> int:
    """Check if string length meets minimum."""
    if len(text) > min_len:
        return 1
    return 0


def test_module() -> int:
    """Demonstrate type extraction for various Python types."""
    ok: int = 0

    r1: int = simple_types_test(1, 2.0, "hello", 1)
    if r1 == 1:
        ok = ok + 1

    items: list[int] = [1, 2, 3]
    r2: int = container_list_test(items)
    if r2 == 6:
        ok = ok + 1

    strs: list[str] = list_to_str_list(items)
    if len(strs) == 3:
        ok = ok + 1

    r3: str = maybe_to_str(42)
    if r3 == "42":
        ok = ok + 1

    r4: str = maybe_to_str(0)
    if r4 == "":
        ok = ok + 1

    matrix: list[list[int]] = [[1, 2], [3, 4]]
    r5: int = nested_list_sum(matrix)
    if r5 == 10:
        ok = ok + 1

    r6: int = first_element(items)
    if r6 == 1:
        ok = ok + 1

    empty: list[int] = []
    r7: int = first_element(empty)
    if r7 == 0:
        ok = ok + 1

    pair: list[int] = make_pair(10, 20)
    if len(pair) == 2:
        ok = ok + 1
    if pair[0] == 10:
        ok = ok + 1

    r8: int = apply_operation(3, 4, 1)
    if r8 == 7:
        ok = ok + 1

    r9: int = apply_operation(3, 4, 2)
    if r9 == 12:
        ok = ok + 1

    ids: list[int] = [1, 2, 3]
    names: list[str] = ["Alice", "Bob", "Carol"]
    name: str = user_lookup(ids, names, 2)
    if name == "Bob":
        ok = ok + 1

    b1: int = int_to_bool(5)
    if b1 == 1:
        ok = ok + 1

    b2: int = int_to_bool(0)
    if b2 == 0:
        ok = ok + 1

    sc: int = str_len_check("hello", 3)
    if sc == 1:
        ok = ok + 1

    return ok
