"""Bracket matching using integer codes instead of string comparisons."""


def char_to_code(c: str) -> int:
    """Map bracket chars to integer codes. Open: 1,2,3  Close: -1,-2,-3."""
    if c == "[":
        return 1
    if c == "]":
        return 0 - 1
    if c == "{":
        return 2
    if c == "}":
        return 0 - 2
    return 0


def is_open_bracket(code: int) -> int:
    """Returns 1 if code represents open bracket."""
    if code > 0:
        return 1
    return 0


def brackets_match(open_code: int, close_code: int) -> int:
    """Returns 1 if open and close codes match."""
    if open_code + close_code == 0:
        return 1
    return 0


def check_balanced(codes: list[int]) -> int:
    """Check if bracket sequence is balanced. Returns 1 if balanced."""
    stack: list[int] = []
    i: int = 0
    while i < len(codes):
        code: int = codes[i]
        if code == 0:
            i = i + 1
            continue
        if is_open_bracket(code) == 1:
            stack.append(code)
        else:
            if len(stack) == 0:
                return 0
            top: int = stack[len(stack) - 1]
            stack.pop()
            if brackets_match(top, code) == 0:
                return 0
        i = i + 1
    if len(stack) == 0:
        return 1
    return 0


def max_nesting_depth(codes: list[int]) -> int:
    """Find maximum nesting depth of brackets."""
    depth: int = 0
    max_depth: int = 0
    i: int = 0
    while i < len(codes):
        code: int = codes[i]
        if is_open_bracket(code) == 1:
            depth = depth + 1
            if depth > max_depth:
                max_depth = depth
        elif code < 0:
            depth = depth - 1
        i = i + 1
    return max_depth


def test_module() -> int:
    """Test bracket matching."""
    ok: int = 0
    b1: list[int] = [1, 2, 0 - 2, 0 - 1]
    if check_balanced(b1) == 1:
        ok = ok + 1
    b2: list[int] = [1, 2, 0 - 1, 0 - 2]
    if check_balanced(b2) == 0:
        ok = ok + 1
    b3: list[int] = [1, 0 - 1]
    if check_balanced(b3) == 1:
        ok = ok + 1
    empty: list[int] = []
    if check_balanced(empty) == 1:
        ok = ok + 1
    if max_nesting_depth(b1) == 2:
        ok = ok + 1
    b4: list[int] = [1, 1, 1, 0 - 1, 0 - 1, 0 - 1]
    if max_nesting_depth(b4) == 3:
        ok = ok + 1
    if brackets_match(1, 0 - 1) == 1:
        ok = ok + 1
    return ok
