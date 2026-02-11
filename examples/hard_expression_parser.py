"""Simple expression evaluation without parentheses (integers, +, -, *)."""


def parse_number(tokens: list[int], pos: int) -> int:
    """Get number at position in token list."""
    return tokens[pos]


def eval_expr(nums: list[int], ops: list[int]) -> int:
    """Evaluate expression given numbers and operators.
    ops: 1=add, 2=sub, 3=mul.
    Handles * first (precedence), then + and -."""
    if len(nums) == 0:
        return 0
    if len(ops) == 0:
        return nums[0]
    n_list: list[int] = []
    o_list: list[int] = []
    i: int = 0
    while i < len(nums):
        n_list.append(nums[i])
        i = i + 1
    i = 0
    while i < len(ops):
        o_list.append(ops[i])
        i = i + 1
    idx: int = 0
    while idx < len(o_list):
        if o_list[idx] == 3:
            product: int = n_list[idx] * n_list[idx + 1]
            n_list[idx] = product
            j: int = idx + 1
            while j < len(n_list) - 1:
                n_list[j] = n_list[j + 1]
                j = j + 1
            n_list.pop()
            j2: int = idx
            while j2 < len(o_list) - 1:
                o_list[j2] = o_list[j2 + 1]
                j2 = j2 + 1
            o_list.pop()
        else:
            idx = idx + 1
    result: int = n_list[0]
    idx = 0
    while idx < len(o_list):
        if o_list[idx] == 1:
            result = result + n_list[idx + 1]
        elif o_list[idx] == 2:
            result = result - n_list[idx + 1]
        idx = idx + 1
    return result


def eval_sum(nums: list[int]) -> int:
    """Evaluate pure sum expression."""
    ops: list[int] = []
    i: int = 0
    while i < len(nums) - 1:
        ops.append(1)
        i = i + 1
    return eval_expr(nums, ops)


def eval_alternating(nums: list[int]) -> int:
    """Evaluate alternating add/sub expression."""
    ops: list[int] = []
    i: int = 0
    while i < len(nums) - 1:
        if i % 2 == 0:
            ops.append(1)
        else:
            ops.append(2)
        i = i + 1
    return eval_expr(nums, ops)


def test_module() -> int:
    """Test expression evaluation."""
    ok: int = 0
    nums1: list[int] = [2, 3, 4]
    ops1: list[int] = [1, 3]
    if eval_expr(nums1, ops1) == 14:
        ok = ok + 1
    nums2: list[int] = [5, 3, 2]
    ops2: list[int] = [2, 1]
    if eval_expr(nums2, ops2) == 4:
        ok = ok + 1
    nums3: list[int] = [2, 3, 4]
    ops3: list[int] = [3, 3]
    if eval_expr(nums3, ops3) == 24:
        ok = ok + 1
    nums4: list[int] = [10]
    ops4: list[int] = []
    if eval_expr(nums4, ops4) == 10:
        ok = ok + 1
    s1: list[int] = [1, 2, 3, 4]
    if eval_sum(s1) == 10:
        ok = ok + 1
    a1: list[int] = [10, 3, 5, 2]
    if eval_alternating(a1) == 10:
        ok = ok + 1
    empty: list[int] = []
    ops_e: list[int] = []
    if eval_expr(empty, ops_e) == 0:
        ok = ok + 1
    return ok
