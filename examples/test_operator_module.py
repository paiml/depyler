"""Operator module patterns using pure functions, no imports.

Tests arithmetic, comparison, logical, bitwise, sequence,
and item access operators using explicit implementations.
"""


def test_arithmetic_ops() -> int:
    """Test arithmetic operator functions."""
    a: int = 10
    b: int = 5
    add_result: int = a + b
    sub_result: int = a - b
    mul_result: int = a * b
    floordiv_result: int = a // b
    mod_result: int = a % b
    pow_result: int = a * a
    return add_result + sub_result + mul_result


def test_comparison_ops() -> int:
    """Test comparison operators. Returns 1 if all pass."""
    a: int = 10
    b: int = 5
    ok: int = 0
    if a > b:
        ok = ok + 1
    if a != b:
        ok = ok + 1
    if a >= b:
        ok = ok + 1
    if b < a:
        ok = ok + 1
    if b <= a:
        ok = ok + 1
    if a == 10:
        ok = ok + 1
    return ok


def test_logical_ops() -> int:
    """Test logical operators. Returns count of passed checks."""
    ok: int = 0
    t: int = 1
    f: int = 0
    and_result: int = 0
    if t == 1:
        if f == 1:
            and_result = 1
    or_result: int = 0
    if t == 1:
        or_result = 1
    if f == 1:
        or_result = 1
    not_result: int = 0
    if t == 0:
        not_result = 1
    if or_result == 1:
        ok = ok + 1
    if and_result == 0:
        ok = ok + 1
    if not_result == 0:
        ok = ok + 1
    return ok


def test_bitwise_ops() -> int:
    """Test bitwise operators."""
    a: int = 12
    b: int = 10
    and_result: int = a & b
    or_result: int = a | b
    xor_result: int = a ^ b
    lshift_result: int = a << 1
    rshift_result: int = a >> 1
    return and_result + or_result


def itemgetter_list(data: list[int], idx: int) -> int:
    """Get item at index from list."""
    return data[idx]


def apply_operation(a: int, b: int, op: str) -> int:
    """Apply operation based on string name."""
    if op == "add":
        return a + b
    if op == "sub":
        return a - b
    if op == "mul":
        return a * b
    if op == "div":
        if b == 0:
            return 0
        return a // b
    return 0


def test_abs_operator() -> int:
    """Test absolute value."""
    neg: int = 0 - 42
    positive: int = abs(neg)
    return positive


def test_neg_operator() -> int:
    """Test negation."""
    positive: int = 42
    negative: int = 0 - positive
    return negative


def test_contains_op(data: list[int], value: int) -> int:
    """Check if value is in list. Returns 1 if found, 0 otherwise."""
    if value in data:
        return 1
    return 0


def concat_lists(list1: list[int], list2: list[int]) -> list[int]:
    """Concatenate two lists manually."""
    result: list[int] = []
    i: int = 0
    while i < len(list1):
        v: int = list1[i]
        result.append(v)
        i = i + 1
    j: int = 0
    while j < len(list2):
        v2: int = list2[j]
        result.append(v2)
        j = j + 1
    return result


def repeat_list(src: list[int], times: int) -> list[int]:
    """Repeat list contents n times."""
    result: list[int] = []
    t: int = 0
    while t < times:
        i: int = 0
        while i < len(src):
            v: int = src[i]
            result.append(v)
            i = i + 1
        t = t + 1
    return result


def setitem_list(data: list[int], idx: int, value: int) -> list[int]:
    """Set item at index in list."""
    data[idx] = value
    return data


def delitem_list(data: list[int], del_idx: int) -> list[int]:
    """Delete item at index from list."""
    new_data: list[int] = []
    i: int = 0
    while i < len(data):
        if i != del_idx:
            v: int = data[i]
            new_data.append(v)
        i = i + 1
    return new_data


def max_by_second(keys: list[int], vals: list[int]) -> int:
    """Find key with max value from parallel lists. Returns key."""
    if len(keys) == 0:
        return 0
    best_key: int = keys[0]
    best_val: int = vals[0]
    i: int = 1
    while i < len(keys):
        v: int = vals[i]
        if v > best_val:
            best_val = v
            best_key = keys[i]
        i = i + 1
    return best_key


def min_by_second(keys: list[int], vals: list[int]) -> int:
    """Find key with min value from parallel lists. Returns key."""
    if len(keys) == 0:
        return 0
    best_key: int = keys[0]
    best_val: int = vals[0]
    i: int = 1
    while i < len(keys):
        v: int = vals[i]
        if v < best_val:
            best_val = v
            best_key = keys[i]
        i = i + 1
    return best_key


def sort_by_value(keys: list[int], vals: list[int]) -> list[int]:
    """Sort keys by their corresponding values (bubble sort)."""
    n: int = len(keys)
    i: int = 0
    while i < n:
        j: int = i + 1
        while j < n:
            vi: int = vals[i]
            vj: int = vals[j]
            if vj < vi:
                tmp_k: int = keys[i]
                keys[i] = keys[j]
                keys[j] = tmp_k
                tmp_v: int = vals[i]
                vals[i] = vals[j]
                vals[j] = tmp_v
            j = j + 1
        i = i + 1
    return keys


def test_truthiness() -> int:
    """Test truth value patterns. Returns count of passed checks."""
    ok: int = 0
    empty_list: list[int] = []
    if len(empty_list) == 0:
        ok = ok + 1
    full_list: list[int] = [1, 2, 3]
    if len(full_list) > 0:
        ok = ok + 1
    return ok


def test_identity() -> int:
    """Test identity/equality patterns."""
    a: int = 42
    b: int = 42
    c: int = 99
    ok: int = 0
    if a == b:
        ok = ok + 1
    if a != c:
        ok = ok + 1
    return ok


def chain_compare(x: int, lo: int, hi: int) -> int:
    """Test chained comparison: lo <= x <= hi. Returns 1 if true."""
    if x >= lo:
        if x <= hi:
            return 1
    return 0


def test_module() -> int:
    """Test all operator features."""
    ok: int = 0

    arith: int = test_arithmetic_ops()
    if arith == 65:
        ok = ok + 1

    comp: int = test_comparison_ops()
    if comp == 6:
        ok = ok + 1

    logic: int = test_logical_ops()
    if logic == 3:
        ok = ok + 1

    bits: int = test_bitwise_ops()
    if bits == 22:
        ok = ok + 1

    data: list[int] = [10, 20, 30, 40, 50]
    item: int = itemgetter_list(data, 2)
    if item == 30:
        ok = ok + 1

    abs_val: int = test_abs_operator()
    if abs_val == 42:
        ok = ok + 1

    neg_val: int = test_neg_operator()
    if neg_val == -42:
        ok = ok + 1

    cont: int = test_contains_op(data, 30)
    if cont == 1:
        ok = ok + 1

    l1: list[int] = [1, 2, 3]
    l2: list[int] = [4, 5, 6]
    cat: list[int] = concat_lists(l1, l2)
    if len(cat) == 6:
        ok = ok + 1
    if cat[5] == 6:
        ok = ok + 1

    src: list[int] = [1, 2]
    rep: list[int] = repeat_list(src, 3)
    if len(rep) == 6:
        ok = ok + 1

    sd: list[int] = [10, 20, 30, 40]
    sd2: list[int] = setitem_list(sd, 2, 99)
    if sd2[2] == 99:
        ok = ok + 1

    dd: list[int] = [10, 20, 30, 40]
    dd2: list[int] = delitem_list(dd, 2)
    if len(dd2) == 3:
        ok = ok + 1

    op_r: int = apply_operation(10, 5, "add")
    if op_r == 15:
        ok = ok + 1

    mk: list[int] = [1, 2, 3]
    mv: list[int] = [100, 50, 200]
    mx: int = max_by_second(mk, mv)
    if mx == 3:
        ok = ok + 1

    mn: int = min_by_second(mk, mv)
    if mn == 2:
        ok = ok + 1

    truth: int = test_truthiness()
    if truth == 2:
        ok = ok + 1

    ident: int = test_identity()
    if ident == 2:
        ok = ok + 1

    ch: int = chain_compare(5, 1, 10)
    if ch == 1:
        ok = ok + 1

    ch2: int = chain_compare(15, 1, 10)
    if ch2 == 0:
        ok = ok + 1

    return ok
