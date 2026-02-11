"""Parentheses matching using stack simulation.

Implements balanced parentheses checking and related
operations using a list as a stack.
"""


def is_balanced(s: str) -> int:
    """Check if string has balanced parentheses. Returns 1 if balanced."""
    stack: list[int] = []
    s_len: int = len(s)
    i: int = 0
    while i < s_len:
        ch: str = s[i]
        if ch == "(":
            stack.append(1)
        elif ch == ")":
            if len(stack) == 0:
                return 0
            stack_len: int = len(stack) - 1
            stack = stack[:stack_len]
        i = i + 1
    if len(stack) == 0:
        return 1
    return 0


def max_nesting_depth(s: str) -> int:
    """Find the maximum nesting depth of parentheses."""
    max_depth: int = 0
    current: int = 0
    s_len: int = len(s)
    i: int = 0
    while i < s_len:
        ch: str = s[i]
        if ch == "(":
            current = current + 1
            if current > max_depth:
                max_depth = current
        elif ch == ")":
            current = current - 1
        i = i + 1
    return max_depth


def count_matched_pairs(s: str) -> int:
    """Count the number of matched parenthesis pairs."""
    depth: int = 0
    pairs: int = 0
    s_len: int = len(s)
    i: int = 0
    while i < s_len:
        ch: str = s[i]
        if ch == "(":
            depth = depth + 1
        elif ch == ")":
            if depth > 0:
                pairs = pairs + 1
                depth = depth - 1
        i = i + 1
    return pairs


def min_removals_to_balance(s: str) -> int:
    """Count minimum removals to make parentheses balanced."""
    open_count: int = 0
    unmatched_close: int = 0
    s_len: int = len(s)
    i: int = 0
    while i < s_len:
        ch: str = s[i]
        if ch == "(":
            open_count = open_count + 1
        elif ch == ")":
            if open_count > 0:
                open_count = open_count - 1
            else:
                unmatched_close = unmatched_close + 1
        i = i + 1
    result: int = open_count + unmatched_close
    return result


def test_module() -> int:
    """Test parentheses matching operations."""
    ok: int = 0

    bal: int = is_balanced("((()))")
    if bal == 1:
        ok = ok + 1

    unbal: int = is_balanced("(()")
    if unbal == 0:
        ok = ok + 1

    depth: int = max_nesting_depth("((())())")
    if depth == 3:
        ok = ok + 1

    pairs: int = count_matched_pairs("(())(")
    if pairs == 2:
        ok = ok + 1

    removals: int = min_removals_to_balance("())(")
    if removals == 2:
        ok = ok + 1

    return ok
