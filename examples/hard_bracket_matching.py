"""Bracket and parenthesis matching algorithms.

Tests: stack-based matching, nested bracket validation.
"""


def is_balanced(s: str) -> bool:
    """Check if brackets are balanced using a stack."""
    stack: list[int] = []
    for ch in s:
        code: int = ord(ch)
        if code == 40 or code == 91 or code == 123:
            stack.append(code)
        elif code == 41:
            if len(stack) == 0:
                return False
            top: int = stack.pop()
            if top != 40:
                return False
        elif code == 93:
            if len(stack) == 0:
                return False
            top2: int = stack.pop()
            if top2 != 91:
                return False
        elif code == 125:
            if len(stack) == 0:
                return False
            top3: int = stack.pop()
            if top3 != 123:
                return False
    return len(stack) == 0


def max_nesting_depth(s: str) -> int:
    """Find maximum nesting depth of parentheses."""
    depth: int = 0
    max_depth: int = 0
    for ch in s:
        if ord(ch) == 40:
            depth += 1
            if depth > max_depth:
                max_depth = depth
        elif ord(ch) == 41:
            depth -= 1
    return max_depth


def count_pairs(s: str) -> int:
    """Count matched bracket pairs."""
    stack: list[int] = []
    pairs: int = 0
    for ch in s:
        code: int = ord(ch)
        if code == 40 or code == 91 or code == 123:
            stack.append(code)
        elif code == 41 and len(stack) > 0 and stack[len(stack) - 1] == 40:
            stack.pop()
            pairs += 1
        elif code == 93 and len(stack) > 0 and stack[len(stack) - 1] == 91:
            stack.pop()
            pairs += 1
        elif code == 125 and len(stack) > 0 and stack[len(stack) - 1] == 123:
            stack.pop()
            pairs += 1
    return pairs


def min_removals_to_balance(s: str) -> int:
    """Minimum removals to make parentheses balanced."""
    open_count: int = 0
    close_count: int = 0
    for ch in s:
        if ord(ch) == 40:
            open_count += 1
        elif ord(ch) == 41:
            if open_count > 0:
                open_count -= 1
            else:
                close_count += 1
    return open_count + close_count


def test_module() -> int:
    """Test bracket matching operations."""
    ok: int = 0

    if is_balanced("()[]{}"):
        ok += 1
    if not is_balanced("([)]"):
        ok += 1
    if is_balanced("{[()]}"):
        ok += 1

    d: int = max_nesting_depth("((()))")
    if d == 3:
        ok += 1

    p: int = count_pairs("(())[{}]")
    if p == 4:
        ok += 1

    r: int = min_removals_to_balance("(()")
    if r == 1:
        ok += 1

    return ok
