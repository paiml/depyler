"""Validate parentheses using stack-based approach."""


def is_valid_parens(s: str) -> int:
    """Return 1 if parentheses string is valid, else 0.
    Supports (, ), [, ], {, }."""
    stack: list[str] = []
    idx: int = 0
    length: int = len(s)
    while idx < length:
        ch: str = s[idx]
        if ch == "(" or ch == "[" or ch == "{":
            stack.append(ch)
        else:
            if len(stack) == 0:
                return 0
            stack_len: int = len(stack)
            top_pos: int = stack_len - 1
            top: str = stack[top_pos]
            if ch == ")" and top != "(":
                return 0
            if ch == "]" and top != "[":
                return 0
            if ch == "}" and top != "{":
                return 0
            stack.pop()
        idx = idx + 1
    if len(stack) == 0:
        return 1
    return 0


def max_nesting_depth(s: str) -> int:
    """Return maximum nesting depth of parentheses."""
    max_depth: int = 0
    current_depth: int = 0
    idx: int = 0
    length: int = len(s)
    while idx < length:
        ch: str = s[idx]
        if ch == "(" or ch == "[" or ch == "{":
            current_depth = current_depth + 1
            if current_depth > max_depth:
                max_depth = current_depth
        elif ch == ")" or ch == "]" or ch == "}":
            current_depth = current_depth - 1
        idx = idx + 1
    return max_depth


def min_removals_to_valid(s: str) -> int:
    """Return minimum removals to make parentheses valid (only round parens)."""
    open_count: int = 0
    close_excess: int = 0
    idx: int = 0
    length: int = len(s)
    while idx < length:
        ch: str = s[idx]
        if ch == "(":
            open_count = open_count + 1
        elif ch == ")":
            if open_count > 0:
                open_count = open_count - 1
            else:
                close_excess = close_excess + 1
        idx = idx + 1
    return open_count + close_excess


def test_module() -> int:
    passed: int = 0

    if is_valid_parens("()[]{}") == 1:
        passed = passed + 1
    if is_valid_parens("([)]") == 0:
        passed = passed + 1
    if is_valid_parens("{[]}") == 1:
        passed = passed + 1
    if is_valid_parens("(") == 0:
        passed = passed + 1

    if max_nesting_depth("((()))") == 3:
        passed = passed + 1
    if max_nesting_depth("()()") == 1:
        passed = passed + 1

    if min_removals_to_valid("())((") == 3:
        passed = passed + 1

    return passed
