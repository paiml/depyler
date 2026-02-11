"""Bracket matching and validation."""


def is_matching_pair(opening: str, closing: str) -> int:
    """Check if two brackets form a matching pair. Returns 1 if yes, 0 if no."""
    if opening == "(" and closing == ")":
        return 1
    if opening == "[" and closing == "]":
        return 1
    if opening == "{" and closing == "}":
        return 1
    return 0


def is_balanced(text: str) -> int:
    """Check if brackets in text are balanced. Returns 1 if yes, 0 if no."""
    stack: list[str] = []
    i: int = 0
    length: int = len(text)
    while i < length:
        ch: str = text[i]
        if ch == "(" or ch == "[" or ch == "{":
            stack.append(ch)
        elif ch == ")" or ch == "]" or ch == "}":
            if len(stack) == 0:
                return 0
            top_idx: int = len(stack) - 1
            top: str = stack[top_idx]
            if is_matching_pair(top, ch) == 0:
                return 0
            stack.pop()
        i = i + 1
    if len(stack) == 0:
        return 1
    return 0


def max_nesting_depth(text: str) -> int:
    """Find the maximum nesting depth of brackets."""
    max_depth: int = 0
    current: int = 0
    i: int = 0
    length: int = len(text)
    while i < length:
        ch: str = text[i]
        if ch == "(" or ch == "[" or ch == "{":
            current = current + 1
            if current > max_depth:
                max_depth = current
        elif ch == ")" or ch == "]" or ch == "}":
            current = current - 1
        i = i + 1
    return max_depth


def count_bracket_pairs(text: str) -> int:
    """Count the number of bracket pairs in balanced text."""
    count: int = 0
    i: int = 0
    length: int = len(text)
    while i < length:
        ch: str = text[i]
        if ch == ")" or ch == "]" or ch == "}":
            count = count + 1
        i = i + 1
    return count


def test_module() -> int:
    """Test bracket matching operations."""
    passed: int = 0

    if is_balanced("()[]{}") == 1:
        passed = passed + 1

    if is_balanced("([{}])") == 1:
        passed = passed + 1

    if is_balanced("([)]") == 0:
        passed = passed + 1

    if is_balanced("(") == 0:
        passed = passed + 1

    if is_balanced("") == 1:
        passed = passed + 1

    if max_nesting_depth("((()))") == 3:
        passed = passed + 1

    if max_nesting_depth("()()()") == 1:
        passed = passed + 1

    if count_bracket_pairs("(()())") == 3:
        passed = passed + 1

    return passed
